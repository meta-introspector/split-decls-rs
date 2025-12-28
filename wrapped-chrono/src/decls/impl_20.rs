macro_rules! deps {
    () => {
        TimeDelta!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl Div < i32 > for TimeDelta { type Output = TimeDelta ; fn div (self , rhs : i32) -> TimeDelta { self . checked_div (rhs) . expect ("`i32` is zero") } }
    };
}

impl_20!()