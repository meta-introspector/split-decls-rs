macro_rules! deps {
    () => {
        Utf8SingleCodeUnitWriter!();
        StrConsumer!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < S : StrConsumer > io :: Write for Utf8SingleCodeUnitWriter < S > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { let s = std :: str :: from_utf8 (buf) . expect ("Input must be valid UTF-8") ; self . str_consumer . consume (s) ; Ok (buf . len ()) } fn flush (& mut self) -> io :: Result < () > { Ok (()) } }
    };
}

impl_57!();