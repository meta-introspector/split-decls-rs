macro_rules! deps {
    () => {
        Result!();
        PackageIdSpec!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl ser :: Serialize for PackageIdSpec { fn serialize < S > (& self , s : S) -> std :: result :: Result < S :: Ok , S :: Error > where S : ser :: Serializer , { self . to_string () . serialize (s) } }
    };
}

impl_6!();