macro_rules! deps {
    () => {
        Result!();
        TomlLockfileSourceId!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl ser :: Serialize for TomlLockfileSourceId { fn serialize < S > (& self , s : S) -> Result < S :: Ok , S :: Error > where S : ser :: Serializer , { s . collect_str (& self . as_url ()) } }
    };
}

impl_50!()