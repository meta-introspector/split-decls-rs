macro_rules! deps {
    () => {
        BytesRef!();
    };
}

macro_rules! impl_263 {
    () => {
        deps!();
        impl UpperHex for BytesRef < '_ > { fn fmt (& self , f : & mut Formatter < '_ >) -> Result { for & b in self . 0 { write ! (f , "{:02X}" , b) ? ; } Ok (()) } }
    };
}

impl_263!();