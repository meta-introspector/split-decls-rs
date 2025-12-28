macro_rules! deps {
    () => {
        Ident!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl fmt :: Display for Ident { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . raw { f . write_str ("r#") ? ; } f . write_str (& * self . name) } }
    };
}

impl_12!()