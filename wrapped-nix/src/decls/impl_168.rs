macro_rules! deps {
    () => {
        TimeSpec!();
    };
}

macro_rules! impl_168 {
    () => {
        deps!();
        impl ops :: Add for TimeSpec { type Output = TimeSpec ; fn add (self , rhs : TimeSpec) -> TimeSpec { TimeSpec :: nanoseconds (self . num_nanoseconds () + rhs . num_nanoseconds ()) } }
    };
}

impl_168!();