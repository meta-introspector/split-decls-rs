macro_rules! deps {
    () => {
        MappedSequenceVisitor!();
    };
}

macro_rules! impl_1054 {
    () => {
        deps!();
        impl < 'de , F , T , R > Visitor < 'de > for MappedSequenceVisitor < T , R , F > where T : Deserialize < 'de > , F : Fn (T) -> Result < R , & 'static str > , { type Value = Vec < R > ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { write ! (formatter , "a sequence") } fn visit_seq < A > (self , mut seq : A) -> Result < Self :: Value , A :: Error > where A : SeqAccess < 'de > , { let mut v = Vec :: new () ; while let Some (elem) = seq . next_element () ? { match (self . f) (elem) { Err (s) => Err (< A :: Error > :: custom (s)) ? , Ok (x) => v . push (x) , } } Ok (v) } }
    };
}

impl_1054!();