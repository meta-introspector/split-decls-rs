macro_rules! deps {
    () => {
        TimeVal!();
    };
}

macro_rules! impl_188 {
    () => {
        deps!();
        impl ops :: Div < i32 > for TimeVal { type Output = TimeVal ; fn div (self , rhs : i32) -> TimeVal { let usec = self . num_microseconds () / i64 :: from (rhs) ; TimeVal :: microseconds (usec) } }
    };
}

impl_188!();