macro_rules! deps {
    () => {
        TimeZone!();
        DateTime!();
    };
}

macro_rules! impl_164 {
    () => {
        deps!();
        impl < Tz : TimeZone > Ord for DateTime < Tz > { fn cmp (& self , other : & DateTime < Tz >) -> Ordering { self . datetime . cmp (& other . datetime) } }
    };
}

impl_164!();