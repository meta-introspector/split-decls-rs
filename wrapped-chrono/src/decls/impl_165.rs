macro_rules! deps {
    () => {
        TimeZone!();
        DateTime!();
    };
}

macro_rules! impl_165 {
    () => {
        deps!();
        impl < Tz : TimeZone > hash :: Hash for DateTime < Tz > { fn hash < H : hash :: Hasher > (& self , state : & mut H) { self . datetime . hash (state) } }
    };
}

impl_165!();