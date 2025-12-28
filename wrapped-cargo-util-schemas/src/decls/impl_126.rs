macro_rules! deps {
    () => {
        Result!();
        ProfilePackageSpec!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl ser :: Serialize for ProfilePackageSpec { fn serialize < S > (& self , s : S) -> Result < S :: Ok , S :: Error > where S : ser :: Serializer , { self . to_string () . serialize (s) } }
    };
}

impl_126!();