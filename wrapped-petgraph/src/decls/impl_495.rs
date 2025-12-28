macro_rules! deps {
    () => {
        FloatMeasure!();
    };
}

macro_rules! impl_495 {
    () => {
        deps!();
        impl FloatMeasure for f32 { fn zero () -> Self { 0. } fn infinite () -> Self { 1. / 0. } fn from_f32 (val : f32) -> Self { val } fn from_f64 (val : f64) -> Self { val as f32 } }
    };
}

impl_495!();