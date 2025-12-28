macro_rules! deps {
    () => {
        NoNI!();
        Avx2Machine!();
    };
}

macro_rules! AVX2 {
    () => {
        deps!();
        pub type AVX2 = Avx2Machine < NoNI > ;
    };
}

AVX2!()