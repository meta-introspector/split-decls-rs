macro_rules! deps {
    () => {
        Utf8Path!();
        Utf8PathVisitor!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < 'a > de :: Visitor < 'a > for Utf8PathVisitor { type Value = & 'a Utf8Path ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("a borrowed UTF-8 path") } fn visit_borrowed_str < E > (self , v : & 'a str) -> Result < Self :: Value , E > where E : de :: Error , { Ok (v . as_ref ()) } fn visit_borrowed_bytes < E > (self , v : & 'a [u8]) -> Result < Self :: Value , E > where E : de :: Error , { std :: str :: from_utf8 (v) . map (AsRef :: as_ref) . map_err (| _ | de :: Error :: invalid_value (de :: Unexpected :: Bytes (v) , & self)) } }
    };
}

impl_8!()