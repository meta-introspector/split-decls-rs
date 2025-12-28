macro_rules! deps {
    () => {
        Wrapping!();
    };
}

macro_rules! impl_438 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl < T : Serialize > Serialize for Wrapping < T > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . 0 . serialize (serializer) } }
    };
}

impl_438!();