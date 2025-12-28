macro_rules! deps {
    () => {
        LenType!();
        String!();
    };
}

macro_rules! impl_345 {
    () => {
        deps!();
        impl < 'de , LenT : LenType , const N : usize > Deserialize < 'de > for String < N , LenT > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { struct ValueVisitor < 'de , LenT : LenType , const N : usize > (PhantomData < (& 'de () , LenT) >) ; impl < 'de , LenT : LenType , const N : usize > de :: Visitor < 'de > for ValueVisitor < 'de , LenT , N > { type Value = String < N , LenT > ; fn expecting (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (formatter , "a string no more than {} bytes long" , N as u64) } fn visit_str < E > (self , v : & str) -> Result < Self :: Value , E > where E : de :: Error , { let mut s = String :: new () ; s . push_str (v) . map_err (| _ | E :: invalid_length (v . len () , & self)) ? ; Ok (s) } fn visit_bytes < E > (self , v : & [u8]) -> Result < Self :: Value , E > where E : de :: Error , { let mut s = String :: new () ; s . push_str (core :: str :: from_utf8 (v) . map_err (| _ | E :: invalid_value (de :: Unexpected :: Bytes (v) , & self)) ? ,) . map_err (| _ | E :: invalid_length (v . len () , & self)) ? ; Ok (s) } } deserializer . deserialize_str (ValueVisitor (PhantomData)) } }
    };
}

impl_345!();