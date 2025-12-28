macro_rules! deps {
    () => {
        DashSet!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl < K , H > Serialize for DashSet < K , H > where K : Serialize + Eq + Hash , H : BuildHasher + Clone , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut seq = serializer . serialize_seq (Some (self . len ())) ? ; for ref_multi in self . iter () { seq . serialize_element (ref_multi . key ()) ? ; } seq . end () } }
    };
}

impl_100!();