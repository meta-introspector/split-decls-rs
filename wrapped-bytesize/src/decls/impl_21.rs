macro_rules! deps {
    () => {
        ByteSize!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < 'de > Deserialize < 'de > for ByteSize { fn deserialize < D > (de : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { struct ByteSizeVisitor ; impl de :: Visitor < '_ > for ByteSizeVisitor { type Value = ByteSize ; fn expecting (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { formatter . write_str ("an integer or string") } fn visit_i64 < E : de :: Error > (self , value : i64) -> Result < Self :: Value , E > { if let Ok (val) = u64 :: try_from (value) { Ok (ByteSize (val)) } else { Err (E :: invalid_value (de :: Unexpected :: Signed (value) , & "integer overflow" ,)) } } fn visit_u64 < E : de :: Error > (self , value : u64) -> Result < Self :: Value , E > { Ok (ByteSize (value)) } fn visit_str < E : de :: Error > (self , value : & str) -> Result < Self :: Value , E > { if let Ok (val) = value . parse () { Ok (val) } else { Err (E :: invalid_value (de :: Unexpected :: Str (value) , & "parsable string" ,)) } } } if de . is_human_readable () { de . deserialize_any (ByteSizeVisitor) } else { de . deserialize_u64 (ByteSizeVisitor) } } }
    };
}

impl_21!();