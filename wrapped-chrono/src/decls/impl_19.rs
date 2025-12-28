macro_rules! deps {
    () => {
        TimeDelta!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl Mul < i32 > for TimeDelta { type Output = TimeDelta ; fn mul (self , rhs : i32) -> TimeDelta { self . checked_mul (rhs) . expect ("`TimeDelta * i32` overflowed") } }
    };
}

impl_19!()