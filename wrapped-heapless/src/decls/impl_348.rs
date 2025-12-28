macro_rules! deps {
    () => {
        IndexSet!();
    };
}

macro_rules! impl_348 {
    () => {
        deps!();
        impl < T , S , const N : usize > Serialize for IndexSet < T , S , N > where T : Eq + Hash + Serialize , S : BuildHasher , { fn serialize < SER > (& self , serializer : SER) -> Result < SER :: Ok , SER :: Error > where SER : Serializer , { let mut seq = serializer . serialize_seq (Some (self . len ())) ? ; for element in self { seq . serialize_element (element) ? ; } seq . end () } }
    };
}

impl_348!();