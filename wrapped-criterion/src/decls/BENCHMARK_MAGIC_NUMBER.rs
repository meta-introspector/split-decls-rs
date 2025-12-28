macro_rules! deps {
    () => {
        Criterion!();
    };
}

macro_rules! BENCHMARK_MAGIC_NUMBER {
    () => {
        deps!();
        const BENCHMARK_MAGIC_NUMBER : & str = "Criterion" ;
    };
}

BENCHMARK_MAGIC_NUMBER!()