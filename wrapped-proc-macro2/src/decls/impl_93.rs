macro_rules! deps {
    () => {
        TokenStream!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl Debug for TokenStream { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . write_str ("TokenStream ") ? ; f . debug_list () . entries (self . clone ()) . finish () } }
    };
}

impl_93!()