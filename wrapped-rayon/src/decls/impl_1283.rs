macro_rules! deps {
    () => {
        ParallelString!();
    };
}

macro_rules! impl_1283 {
    () => {
        deps!();
        impl ParallelString for str { # [inline] fn as_parallel_string (& self) -> & str { self } }
    };
}

impl_1283!()