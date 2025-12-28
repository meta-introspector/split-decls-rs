macro_rules! deps {
    () => {
        HistoryBuf!();
    };
}

macro_rules! impl_342 {
    () => {
        deps!();
        impl < 'de , T , const N : usize > Deserialize < 'de > for HistoryBuf < T , N > where T : Deserialize < 'de > , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { struct ValueVisitor < 'de , T , const N : usize > (PhantomData < (& 'de () , T) >) ; impl < 'de , T , const N : usize > serde_core :: de :: Visitor < 'de > for ValueVisitor < 'de , T , N > where T : Deserialize < 'de > , { type Value = HistoryBuf < T , N > ; fn expecting (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { formatter . write_str ("a sequence") } fn visit_seq < A > (self , mut seq : A) -> Result < Self :: Value , A :: Error > where A : SeqAccess < 'de > , { let mut values = HistoryBuf :: new () ; while let Some (value) = seq . next_element () ? { values . write (value) ; } Ok (values) } } deserializer . deserialize_seq (ValueVisitor (PhantomData)) } }
    };
}

impl_342!();