macro_rules! deps {
    () => {
        TimeVal!();
    };
}

macro_rules! impl_186 {
    () => {
        deps!();
        impl ops :: Sub for TimeVal { type Output = TimeVal ; fn sub (self , rhs : TimeVal) -> TimeVal { TimeVal :: microseconds (self . num_microseconds () - rhs . num_microseconds ()) } }
    };
}

impl_186!();