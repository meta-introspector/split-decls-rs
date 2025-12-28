macro_rules! deps {
    () => {
        PotentialUtf8!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        # [doc = " This impl requires enabling the optional `serde` Cargo feature"] # [cfg (feature = "serde")] impl serde_core :: Serialize for PotentialUtf8 { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde_core :: Serializer , { use serde_core :: ser :: Error ; let s = self . try_as_str () . map_err (| _ | S :: Error :: custom ("invalid UTF-8 in PotentialUtf8")) ? ; if serializer . is_human_readable () { serializer . serialize_str (s) } else { serializer . serialize_bytes (s . as_bytes ()) } } }
    };
}

impl_33!()