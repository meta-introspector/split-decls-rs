macro_rules! deps {
    () => {
        ListFormat!();
        Mode!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl Mode { pub fn is_benchmark (& self) -> bool { matches ! (self , Mode :: Benchmark) } pub fn is_terse (& self) -> bool { matches ! (self , Mode :: List (ListFormat :: Terse)) } }
    };
}

impl_34!()