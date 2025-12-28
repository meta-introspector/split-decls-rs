macro_rules! deps {
    () => {
        IndexMap!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < K , V , S > Serialize for IndexMap < K , V , S > where K : Serialize , V : Serialize , { fn serialize < T > (& self , serializer : T) -> Result < T :: Ok , T :: Error > where T : Serializer , { serializer . collect_map (self) } }
    };
}

impl_22!()