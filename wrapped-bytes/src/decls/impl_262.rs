macro_rules! deps {
    () => {
        BytesRef!();
    };
}

macro_rules! impl_262 {
    () => {
        deps!();
        impl LowerHex for BytesRef < '_ > { fn fmt (& self , f : & mut Formatter < '_ >) -> Result { for & b in self . 0 { write ! (f , "{:02x}" , b) ? ; } Ok (()) } }
    };
}

impl_262!();