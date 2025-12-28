macro_rules! deps {
    () => {
        Platform!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl serde_core :: Serialize for Platform { fn serialize < S > (& self , s : S) -> Result < S :: Ok , S :: Error > where S : serde_core :: Serializer , { self . to_string () . serialize (s) } }
    };
}

impl_35!();