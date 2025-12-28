macro_rules! deps {
    () => {
        TimeZone!();
        Date!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < Tz : TimeZone > Ord for Date < Tz > { fn cmp (& self , other : & Date < Tz >) -> Ordering { self . date . cmp (& other . date) } }
    };
}

impl_46!()