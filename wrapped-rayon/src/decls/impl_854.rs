macro_rules! deps {
    () => {
        StepBy!();
    };
}

macro_rules! impl_854 {
    () => {
        deps!();
        impl < I > StepBy < I > { # [doc = " Creates a new `StepBy` iterator."] pub (super) fn new (base : I , step : usize) -> Self { StepBy { base , step } } }
    };
}

impl_854!();