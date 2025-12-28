macro_rules! deps {
    () => {
        TimeDelta!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl Sub for TimeDelta { type Output = TimeDelta ; fn sub (self , rhs : TimeDelta) -> TimeDelta { self . checked_sub (& rhs) . expect ("`TimeDelta - TimeDelta` overflowed") } }
    };
}

impl_16!()