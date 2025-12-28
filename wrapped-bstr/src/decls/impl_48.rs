macro_rules! deps {
    () => {
        EscapeBytes!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < 'a > core :: fmt :: Display for EscapeBytes < 'a > { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { use core :: fmt :: Write ; for ch in self . clone () { f . write_char (ch) ? ; } Ok (()) } }
    };
}

impl_48!();