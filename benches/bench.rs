use criterion::{criterion_group, criterion_main, Criterion};
use rand::{thread_rng, Rng};
use rgeometry::data::*;
use rgeometry::PolygonScalar;

fn rand_point<R: Rng>(rng: &mut R) -> Point<f64> {
  Point::new([rng.gen_range(-100.0..100.0), rng.gen_range(-100.0..100.0)])
}

pub fn criterion_benchmark(c: &mut Criterion) {
  let mut rng = thread_rng();
  let p0 = rand_point(&mut rng);
  let p1 = rand_point(&mut rng);
  let p2 = rand_point(&mut rng);

  c.bench_function("PolygonScalar::cmp_slope", |b| {
    b.iter(|| PolygonScalar::cmp_slope(&p0, &p1, &p2))
  });

  c.bench_function("PolygonScalar::cmp_dist", |b| {
    b.iter(|| PolygonScalar::cmp_dist(&p0, &p1, &p2))
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
