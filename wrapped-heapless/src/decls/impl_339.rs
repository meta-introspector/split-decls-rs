macro_rules! deps {
    () => {
        IndexSet!();
    };
}

macro_rules! impl_339 {
    () => {
        deps!();
        impl < 'de , T , S , const N : usize > Deserialize < 'de > for IndexSet < T , BuildHasherDefault < S > , N > where T : Eq + Hash + Deserialize < 'de > , S : Hasher + Default , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { struct ValueVisitor < 'de , T , S , const N : usize > (PhantomData < (& 'de () , T , S) >) ; impl < 'de , T , S , const N : usize > de :: Visitor < 'de > for ValueVisitor < 'de , T , S , N > where T : Eq + Hash + Deserialize < 'de > , S : Hasher + Default , { type Value = IndexSet < T , BuildHasherDefault < S > , N > ; fn expecting (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { formatter . write_str ("a sequence") } fn visit_seq < A > (self , mut seq : A) -> Result < Self :: Value , A :: Error > where A : SeqAccess < 'de > , { let mut values = IndexSet :: new () ; while let Some (value) = seq . next_element () ? { if values . insert (value) . is_err () { return Err (A :: Error :: invalid_length (values . capacity () + 1 , & self)) ? ; } } Ok (values) } } deserializer . deserialize_seq (ValueVisitor (PhantomData)) } }
    };
}

impl_339!();