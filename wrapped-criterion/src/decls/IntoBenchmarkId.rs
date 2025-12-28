macro_rules! deps {
    () => {
        BenchmarkId!();
    };
}

macro_rules! IntoBenchmarkId {
    () => {
        deps!();
        # [doc = " Sealed trait which allows users to automatically convert strings to benchmark IDs."] pub trait IntoBenchmarkId : private :: Sealed { fn into_benchmark_id (self) -> BenchmarkId ; }
    };
}

IntoBenchmarkId!();