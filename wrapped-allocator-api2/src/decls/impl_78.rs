macro_rules! deps {
    () => {
        Allocator!();
        Box!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl < T , A > serde :: Serialize for Box < T , A > where T : serde :: Serialize , A : Allocator , { # [inline (always)] fn serialize < S : serde :: ser :: Serializer > (& self , serializer : S) -> Result < S :: Ok , S :: Error > { (* * self) . serialize (serializer) } }
    };
}

impl_78!();