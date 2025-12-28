macro_rules! deps {
    () => {
        Interval!();
        IntervalSet!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        impl < I : Interval > PartialEq for IntervalSet < I > { fn eq (& self , other : & IntervalSet < I >) -> bool { self . ranges . eq (& other . ranges) } }
    };
}

impl_146!()