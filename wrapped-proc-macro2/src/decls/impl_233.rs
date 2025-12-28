macro_rules! deps {
    () => {
        Ident!();
    };
}

macro_rules! impl_233 {
    () => {
        deps!();
        impl Debug for Ident { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { Debug :: fmt (& self . inner , f) } }
    };
}

impl_233!()