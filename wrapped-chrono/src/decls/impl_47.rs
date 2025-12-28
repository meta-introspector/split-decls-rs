macro_rules! deps {
    () => {
        Date!();
        TimeZone!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl < Tz : TimeZone > hash :: Hash for Date < Tz > { fn hash < H : hash :: Hasher > (& self , state : & mut H) { self . date . hash (state) } }
    };
}

impl_47!();