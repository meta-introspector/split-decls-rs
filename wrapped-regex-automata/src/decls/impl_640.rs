macro_rules! deps {
    () => {
        DebugHaystack!();
    };
}

macro_rules! impl_640 {
    () => {
        deps!();
        impl < 'a > core :: fmt :: Debug for DebugHaystack < 'a > { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (f , "\"") ? ; let mut bytes = self . 0 ; while let Some (result) = utf8 :: decode (bytes) { let ch = match result { Ok (ch) => ch , Err (byte) => { write ! (f , r"\x{byte:02x}") ? ; bytes = & bytes [1 ..] ; continue ; } } ; bytes = & bytes [ch . len_utf8 () ..] ; match ch { '\0' => write ! (f , "\\0") ? , '\x01' ..= '\x08' | '\x0b' | '\x0c' | '\x0e' ..= '\x19' | '\x7f' => { write ! (f , "\\x{:02x}" , u32 :: from (ch)) ? ; } '\n' | '\r' | '\t' | _ => { write ! (f , "{}" , ch . escape_debug ()) ? ; } } } write ! (f , "\"") ? ; Ok (()) } }
    };
}

impl_640!()