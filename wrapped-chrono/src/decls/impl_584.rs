macro_rules! deps {
    () => {
        LeapSecond!();
    };
}

macro_rules! impl_584 {
    () => {
        deps!();
        impl LeapSecond { # [doc = " Construct a TZif file leap second"] pub (super) const fn new (unix_leap_time : i64 , correction : i32) -> Self { Self { unix_leap_time , correction } } # [doc = " Returns Unix leap time"] const fn unix_leap_time (& self) -> i64 { self . unix_leap_time } }
    };
}

impl_584!()