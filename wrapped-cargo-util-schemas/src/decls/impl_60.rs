macro_rules! deps {
    () => {
        Result!();
        TomlLockfilePackageId!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl ser :: Serialize for TomlLockfilePackageId { fn serialize < S > (& self , s : S) -> Result < S :: Ok , S :: Error > where S : ser :: Serializer , { s . collect_str (self) } }
    };
}

impl_60!()