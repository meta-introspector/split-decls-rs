macro_rules! deps {
    () => {
        Ident!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl Display for Ident { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { if self . raw { f . write_str ("r#") ? ; } f . write_str (& self . sym) } }
    };
}

impl_126!();