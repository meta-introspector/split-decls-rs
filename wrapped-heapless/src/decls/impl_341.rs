macro_rules! deps {
    () => {
        Deque!();
    };
}

macro_rules! impl_341 {
    () => {
        deps!();
        impl < 'de , T , const N : usize > Deserialize < 'de > for Deque < T , N > where T : Deserialize < 'de > , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { struct ValueVisitor < 'de , T , const N : usize > (PhantomData < (& 'de () , T) >) ; impl < 'de , T , const N : usize > serde_core :: de :: Visitor < 'de > for ValueVisitor < 'de , T , N > where T : Deserialize < 'de > , { type Value = Deque < T , N > ; fn expecting (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { formatter . write_str ("a sequence") } fn visit_seq < A > (self , mut seq : A) -> Result < Self :: Value , A :: Error > where A : SeqAccess < 'de > , { let mut values = Deque :: new () ; while let Some (value) = seq . next_element () ? { if values . push_back (value) . is_err () { return Err (A :: Error :: invalid_length (values . capacity () + 1 , & self)) ? ; } } Ok (values) } } deserializer . deserialize_seq (ValueVisitor (PhantomData)) } }
    };
}

impl_341!()