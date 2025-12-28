macro_rules! deps {
    () => {
        QueryPathNode!();
        Error!();
        Result!();
    };
}

macro_rules! impl_338 {
    () => {
        deps!();
        impl serde :: Serialize for QueryPathNode < '_ > { fn serialize < S : Serializer > (& self , serializer : S) -> Result < S :: Ok , S :: Error > { let mut seq = serializer . serialize_seq (None) ? ; self . try_for_each (| segment | seq . serialize_element (segment)) ? ; seq . end () } }
    };
}

impl_338!()