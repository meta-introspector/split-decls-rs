macro_rules! deps {
    () => {
        IndexMap!();
    };
}

macro_rules! impl_352 {
    () => {
        deps!();
        impl < K , V , S , const N : usize > Serialize for IndexMap < K , V , S , N > where K : Eq + Hash + Serialize , S : BuildHasher , V : Serialize , { fn serialize < SER > (& self , serializer : SER) -> Result < SER :: Ok , SER :: Error > where SER : Serializer , { let mut map = serializer . serialize_map (Some (self . len ())) ? ; for (k , v) in self { map . serialize_entry (k , v) ? ; } map . end () } }
    };
}

impl_352!()