macro_rules! deps {
    () => {
        TimeVal!();
    };
}

macro_rules! impl_187 {
    () => {
        deps!();
        impl ops :: Mul < i32 > for TimeVal { type Output = TimeVal ; fn mul (self , rhs : i32) -> TimeVal { let usec = self . num_microseconds () . checked_mul (i64 :: from (rhs)) . expect ("TimeVal multiply out of bounds") ; TimeVal :: microseconds (usec) } }
    };
}

impl_187!();