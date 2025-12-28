macro_rules! deps {
    () => {
        TimeSpec!();
    };
}

macro_rules! impl_170 {
    () => {
        deps!();
        impl ops :: Mul < i32 > for TimeSpec { type Output = TimeSpec ; fn mul (self , rhs : i32) -> TimeSpec { let usec = self . num_nanoseconds () . checked_mul (i64 :: from (rhs)) . expect ("TimeSpec multiply out of bounds") ; TimeSpec :: nanoseconds (usec) } }
    };
}

impl_170!()