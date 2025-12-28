macro_rules! deps {
    () => {
        Zero!();
        Odd!();
    };
}

macro_rules! impl_254 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl < T : Serialize + Zero > Serialize for Odd < T > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . 0 . serialize (serializer) } }
    };
}

impl_254!()