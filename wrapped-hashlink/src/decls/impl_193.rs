macro_rules! deps {
    () => {
        LinkedHashMap!();
    };
}

macro_rules! impl_193 {
    () => {
        deps!();
        impl < K , V , S > Serialize for LinkedHashMap < K , V , S > where K : Serialize + Eq + Hash , V : Serialize , S : BuildHasher , { # [inline] fn serialize < T : Serializer > (& self , serializer : T) -> Result < T :: Ok , T :: Error > { let mut map_serializer = serializer . serialize_map (Some (self . len ())) ? ; for (k , v) in self { map_serializer . serialize_key (k) ? ; map_serializer . serialize_value (v) ? ; } map_serializer . end () } }
    };
}

impl_193!();