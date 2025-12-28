macro_rules! deps {
    () => {
        TimeDelta!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl Add for TimeDelta { type Output = TimeDelta ; fn add (self , rhs : TimeDelta) -> TimeDelta { self . checked_add (& rhs) . expect ("`TimeDelta + TimeDelta` overflowed") } }
    };
}

impl_15!();