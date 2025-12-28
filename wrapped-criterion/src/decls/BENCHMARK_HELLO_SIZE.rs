macro_rules! BENCHMARK_HELLO_SIZE {
    () => {
        const BENCHMARK_HELLO_SIZE : usize = BENCHMARK_MAGIC_NUMBER . len () + (size_of :: < u8 > () * 3) + size_of :: < u16 > () + size_of :: < u16 > () ;
    };
}

BENCHMARK_HELLO_SIZE!();