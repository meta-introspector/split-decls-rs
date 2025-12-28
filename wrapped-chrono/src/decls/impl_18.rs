macro_rules! deps {
    () => {
        TimeDelta!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl SubAssign for TimeDelta { fn sub_assign (& mut self , rhs : TimeDelta) { let new = self . checked_sub (& rhs) . expect ("`TimeDelta - TimeDelta` overflowed") ; * self = new ; } }
    };
}

impl_18!();