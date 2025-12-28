macro_rules! deps {
    () => {
        Result!();
        Error!();
        Value!();
        RawValue!();
        ReferenceFromString!();
        Formatter!();
    };
}

macro_rules! impl_623 {
    () => {
        deps!();
        impl < 'de > Visitor < 'de > for ReferenceFromString { type Value = & 'de RawValue ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("raw value") } fn visit_borrowed_str < E > (self , s : & 'de str) -> Result < Self :: Value , E > where E : de :: Error , { Ok (RawValue :: from_borrowed (s)) } }
    };
}

impl_623!();