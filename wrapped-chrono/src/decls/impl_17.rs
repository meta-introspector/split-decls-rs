macro_rules! deps {
    () => {
        TimeDelta!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl AddAssign for TimeDelta { fn add_assign (& mut self , rhs : TimeDelta) { let new = self . checked_add (& rhs) . expect ("`TimeDelta + TimeDelta` overflowed") ; * self = new ; } }
    };
}

impl_17!();