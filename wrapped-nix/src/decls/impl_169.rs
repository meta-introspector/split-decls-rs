macro_rules! deps {
    () => {
        TimeSpec!();
    };
}

macro_rules! impl_169 {
    () => {
        deps!();
        impl ops :: Sub for TimeSpec { type Output = TimeSpec ; fn sub (self , rhs : TimeSpec) -> TimeSpec { TimeSpec :: nanoseconds (self . num_nanoseconds () - rhs . num_nanoseconds ()) } }
    };
}

impl_169!();