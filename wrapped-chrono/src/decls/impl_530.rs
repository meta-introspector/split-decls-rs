macro_rules! deps {
    () => {
        Days!();
    };
}

macro_rules! impl_530 {
    () => {
        deps!();
        impl Days { # [doc = " Construct a new `Days` from a number of days"] pub const fn new (num : u64) -> Self { Self (num) } }
    };
}

impl_530!();