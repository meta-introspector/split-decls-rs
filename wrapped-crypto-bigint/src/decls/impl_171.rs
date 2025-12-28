macro_rules! deps {
    () => {
        Limb!();
    };
}

macro_rules! impl_171 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl Serialize for Limb { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . 0 . serialize (serializer) } }
    };
}

impl_171!();