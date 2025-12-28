macro_rules! deps {
    () => {
        CurlSubtransport!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl Read for CurlSubtransport { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { if self . reader . is_none () { self . execute (& []) ? ; } self . reader . as_mut () . unwrap () . read (buf) } }
    };
}

impl_6!();