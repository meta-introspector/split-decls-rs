macro_rules! deps {
    () => {
        IndexMap!();
    };
}

macro_rules! impl_343 {
    () => {
        deps!();
        impl < 'de , K , V , S , const N : usize > Deserialize < 'de > for IndexMap < K , V , BuildHasherDefault < S > , N > where K : Eq + Hash + Deserialize < 'de > , V : Deserialize < 'de > , S : Default + Hasher , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { struct ValueVisitor < 'de , K , V , S , const N : usize > (PhantomData < (& 'de () , K , V , S) >) ; impl < 'de , K , V , S , const N : usize > de :: Visitor < 'de > for ValueVisitor < 'de , K , V , S , N > where K : Eq + Hash + Deserialize < 'de > , V : Deserialize < 'de > , S : Default + Hasher , { type Value = IndexMap < K , V , BuildHasherDefault < S > , N > ; fn expecting (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { formatter . write_str ("a map") } fn visit_map < A > (self , mut map : A) -> Result < Self :: Value , A :: Error > where A : MapAccess < 'de > , { let mut values = IndexMap :: new () ; while let Some ((key , value)) = map . next_entry () ? { if values . insert (key , value) . is_err () { return Err (A :: Error :: invalid_length (values . capacity () + 1 , & self)) ? ; } } Ok (values) } } deserializer . deserialize_map (ValueVisitor (PhantomData)) } }
    };
}

impl_343!()