macro_rules! deps {
    () => {
        Result!();
        Formatter!();
        Value!();
        BoxedFromString!();
        Error!();
        RawValue!();
    };
}

macro_rules! impl_626 {
    () => {
        deps!();
        impl < 'de > Visitor < 'de > for BoxedFromString { type Value = Box < RawValue > ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("raw value") } fn visit_str < E > (self , s : & str) -> Result < Self :: Value , E > where E : de :: Error , { Ok (RawValue :: from_owned (s . to_owned () . into_boxed_str ())) } # [cfg (any (feature = "std" , feature = "alloc"))] fn visit_string < E > (self , s : String) -> Result < Self :: Value , E > where E : de :: Error , { Ok (RawValue :: from_owned (s . into_boxed_str ())) } }
    };
}

impl_626!()