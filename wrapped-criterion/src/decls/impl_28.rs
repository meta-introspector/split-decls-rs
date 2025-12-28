macro_rules! deps {
    () => {
        IntoBenchmarkId!();
        BenchmarkId!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl IntoBenchmarkId for BenchmarkId { fn into_benchmark_id (self) -> BenchmarkId { self } }
    };
}

impl_28!();