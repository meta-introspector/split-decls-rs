macro_rules! serialize_crate_name {
    () => {
        fn serialize_crate_name < S > (name : & CrateName , se : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { se . serialize_str (name . as_str ()) }
    };
}

serialize_crate_name!()