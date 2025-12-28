macro_rules! deps {
    () => {
        Mode!();
        ListFormat!();
    };
}

macro_rules! impl_397 {
    () => {
        deps!();
        impl Mode { pub fn is_benchmark (& self) -> bool { matches ! (self , Mode :: Benchmark) } pub fn is_terse (& self) -> bool { matches ! (self , Mode :: List (ListFormat :: Terse)) } }
    };
}

impl_397!();