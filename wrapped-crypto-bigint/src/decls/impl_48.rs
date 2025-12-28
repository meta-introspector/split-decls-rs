macro_rules! deps {
    () => {
        Checked!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl < T : Copy + Serialize > Serialize for Checked < T > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { Option :: < T > :: from (self . 0) . serialize (serializer) } }
    };
}

impl_48!();