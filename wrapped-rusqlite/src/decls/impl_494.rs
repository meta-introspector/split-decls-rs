macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_494 {
    () => {
        deps!();
        impl From < f64 > for Value { # [inline] fn from (f : f64) -> Self { Self :: Real (f) } }
    };
}

impl_494!();