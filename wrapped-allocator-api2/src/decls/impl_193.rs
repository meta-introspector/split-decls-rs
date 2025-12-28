macro_rules! deps {
    () => {
        Vec!();
        Allocator!();
    };
}

macro_rules! impl_193 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl < T , A > serde :: Serialize for Vec < T , A > where T : serde :: Serialize , A : Allocator , { # [inline (always)] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: ser :: Serializer , { serializer . collect_seq (self) } }
    };
}

impl_193!();