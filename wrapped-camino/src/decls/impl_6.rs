macro_rules! deps {
    () => {
        Utf8PathBuf!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl Serialize for Utf8PathBuf { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . as_str () . serialize (serializer) } }
    };
}

impl_6!();