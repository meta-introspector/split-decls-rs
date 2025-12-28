macro_rules! deps {
    () => {
        LinkedHashSet!();
    };
}

macro_rules! impl_195 {
    () => {
        deps!();
        impl < T , S > Serialize for LinkedHashSet < T , S > where T : Serialize + Eq + Hash , S : BuildHasher , { # [inline] fn serialize < U : Serializer > (& self , serializer : U) -> Result < U :: Ok , U :: Error > { let mut seq_serializer = serializer . serialize_seq (Some (self . len ())) ? ; for v in self { seq_serializer . serialize_element (v) ? ; } seq_serializer . end () } }
    };
}

impl_195!();