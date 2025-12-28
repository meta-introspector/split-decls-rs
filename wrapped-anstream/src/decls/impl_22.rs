macro_rules! deps {
    () => {
        VtUtf8Receiver!();
        Utf8Parser!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl Utf8Parser { fn add (& mut self , byte : u8) -> bool { let mut b = false ; let mut receiver = VtUtf8Receiver (& mut b) ; self . utf8_parser . advance (& mut receiver , byte) ; b } }
    };
}

impl_22!();