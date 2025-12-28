macro_rules! deps {
    () => {
        DateTime!();
        TimeZone!();
    };
}

macro_rules! impl_161 {
    () => {
        deps!();
        impl < Tz : TimeZone , Tz2 : TimeZone > PartialEq < DateTime < Tz2 > > for DateTime < Tz > { fn eq (& self , other : & DateTime < Tz2 >) -> bool { self . datetime == other . datetime } }
    };
}

impl_161!()