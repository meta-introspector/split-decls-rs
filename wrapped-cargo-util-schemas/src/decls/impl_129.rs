macro_rules! deps {
    () => {
        Result!();
        TomlOptLevel!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl ser :: Serialize for TomlOptLevel { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : ser :: Serializer , { match self . 0 . parse :: < u32 > () { Ok (n) => n . serialize (serializer) , Err (_) => self . 0 . serialize (serializer) , } } }
    };
}

impl_129!();