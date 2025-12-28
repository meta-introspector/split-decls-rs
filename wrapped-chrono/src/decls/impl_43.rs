macro_rules! deps {
    () => {
        Date!();
        TimeZone!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl < Tz : TimeZone , Tz2 : TimeZone > PartialEq < Date < Tz2 > > for Date < Tz > { fn eq (& self , other : & Date < Tz2 >) -> bool { self . date == other . date } }
    };
}

impl_43!();