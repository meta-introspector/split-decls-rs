macro_rules! deps {
    () => {
        TimeZone!();
        Date!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < Tz : TimeZone > PartialOrd for Date < Tz > { fn partial_cmp (& self , other : & Date < Tz >) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_45!()