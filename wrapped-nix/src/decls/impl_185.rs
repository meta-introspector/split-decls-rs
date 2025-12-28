macro_rules! deps {
    () => {
        TimeVal!();
    };
}

macro_rules! impl_185 {
    () => {
        deps!();
        impl ops :: Add for TimeVal { type Output = TimeVal ; fn add (self , rhs : TimeVal) -> TimeVal { TimeVal :: microseconds (self . num_microseconds () + rhs . num_microseconds ()) } }
    };
}

impl_185!();