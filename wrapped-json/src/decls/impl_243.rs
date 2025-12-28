macro_rules! deps {
    () => {
        Formatter!();
        Result!();
        Value!();
        Error!();
    };
}

macro_rules! impl_243 {
    () => {
        deps!();
        impl Display for Value { # [doc = " Display a JSON value as a string."] # [doc = ""] # [doc = " ```"] # [doc = " # use serde_json::json;"] # [doc = " #"] # [doc = " let json = json!({ \"city\": \"London\", \"street\": \"10 Downing Street\" });"] # [doc = ""] # [doc = " // Compact format:"] # [doc = " //"] # [doc = " // {\"city\":\"London\",\"street\":\"10 Downing Street\"}"] # [doc = " let compact = format!(\"{}\", json);"] # [doc = " assert_eq!(compact,"] # [doc = "     \"{\\\"city\\\":\\\"London\\\",\\\"street\\\":\\\"10 Downing Street\\\"}\");"] # [doc = ""] # [doc = " // Pretty format:"] # [doc = " //"] # [doc = " // {"] # [doc = " //   \"city\": \"London\","] # [doc = " //   \"street\": \"10 Downing Street\""] # [doc = " // }"] # [doc = " let pretty = format!(\"{:#}\", json);"] # [doc = " assert_eq!(pretty,"] # [doc = "     \"{\\n  \\\"city\\\": \\\"London\\\",\\n  \\\"street\\\": \\\"10 Downing Street\\\"\\n}\");"] # [doc = " ```"] fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { struct WriterFormatter < 'a , 'b : 'a > { inner : & 'a mut fmt :: Formatter < 'b > , } impl < 'a , 'b > io :: Write for WriterFormatter < 'a , 'b > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { let s = unsafe { str :: from_utf8_unchecked (buf) } ; tri ! (self . inner . write_str (s) . map_err (io_error)) ; Ok (buf . len ()) } fn flush (& mut self) -> io :: Result < () > { Ok (()) } } fn io_error (_ : fmt :: Error) -> io :: Error { io :: Error :: new (io :: ErrorKind :: Other , "fmt error") } let alternate = f . alternate () ; let mut wr = WriterFormatter { inner : f } ; if alternate { super :: ser :: to_writer_pretty (& mut wr , self) . map_err (| _ | fmt :: Error) } else { super :: ser :: to_writer (& mut wr , self) . map_err (| _ | fmt :: Error) } } }
    };
}

impl_243!()