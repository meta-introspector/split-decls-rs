macro_rules! deps {
    () => {
        PotentialCodePoint!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        # [doc = " This impl requires enabling the optional `serde` Cargo feature"] # [cfg (feature = "serde")] impl serde_core :: Serialize for PotentialCodePoint { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde_core :: Serializer , { use serde_core :: ser :: Error ; let c = self . try_to_char () . map_err (| _ | S :: Error :: custom ("invalid Unicode scalar value in PotentialCodePoint")) ? ; if serializer . is_human_readable () { serializer . serialize_char (c) } else { self . 0 . serialize (serializer) } } }
    };
}

impl_16!()