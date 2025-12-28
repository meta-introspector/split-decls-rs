macro_rules! deps {
    () => {
        ArrayString!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        # [cfg (feature = "serde")] # [doc = " Requires crate feature `\"serde\"`"] impl < const CAP : usize > Serialize for ArrayString < CAP > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . serialize_str (& * self) } }
    };
}

impl_29!();