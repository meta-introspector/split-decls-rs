macro_rules! deps {
    () => {
        ArrayVec!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        # [cfg (feature = "serde")] # [doc = " Requires crate feature `\"serde\"`"] impl < T : Serialize , const CAP : usize > Serialize for ArrayVec < T , CAP > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . collect_seq (self) } }
    };
}

impl_86!();