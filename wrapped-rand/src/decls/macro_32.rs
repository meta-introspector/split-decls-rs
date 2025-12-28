macro_rules! macro_32 {
    () => {
        float_impls ! { , f64 , u64 , f64 , u64 , 52 , 1023 }
    };
}

macro_32!();