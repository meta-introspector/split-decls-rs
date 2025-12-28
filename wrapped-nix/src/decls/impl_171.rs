macro_rules! deps {
    () => {
        TimeSpec!();
    };
}

macro_rules! impl_171 {
    () => {
        deps!();
        impl ops :: Div < i32 > for TimeSpec { type Output = TimeSpec ; fn div (self , rhs : i32) -> TimeSpec { let usec = self . num_nanoseconds () / i64 :: from (rhs) ; TimeSpec :: nanoseconds (usec) } }
    };
}

impl_171!()