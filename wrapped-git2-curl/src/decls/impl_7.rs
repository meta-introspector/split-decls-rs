macro_rules! deps {
    () => {
        CurlSubtransport!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl Write for CurlSubtransport { fn write (& mut self , data : & [u8]) -> io :: Result < usize > { if self . reader . is_none () { self . execute (data) ? ; } Ok (data . len ()) } fn flush (& mut self) -> io :: Result < () > { Ok (()) } }
    };
}

impl_7!()