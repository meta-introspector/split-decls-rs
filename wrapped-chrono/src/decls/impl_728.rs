macro_rules! deps {
    () => {
        Months!();
    };
}

macro_rules! impl_728 {
    () => {
        deps!();
        impl Months { # [doc = " Construct a new `Months` from a number of months"] pub const fn new (num : u32) -> Self { Self (num) } # [doc = " Returns the total number of months in the `Months` instance."] # [inline] pub const fn as_u32 (& self) -> u32 { self . 0 } }
    };
}

impl_728!();