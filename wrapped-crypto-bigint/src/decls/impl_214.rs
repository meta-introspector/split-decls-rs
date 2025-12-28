macro_rules! deps {
    () => {
        Zero!();
        NonZero!();
    };
}

macro_rules! impl_214 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl < T : Serialize + Zero > Serialize for NonZero < T > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . 0 . serialize (serializer) } }
    };
}

impl_214!();