macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl < 'a > fmt :: Debug for Iter < 'a > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_list () . entries (self . clone () . map (String :: from_utf8_lossy)) . finish () } }
    };
}

impl_109!();