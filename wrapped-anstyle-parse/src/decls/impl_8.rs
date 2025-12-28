macro_rules! deps {
    () => {
        Params!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl Debug for Params { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { write ! (f , "[") ? ; for (i , param) in self . iter () . enumerate () { if i != 0 { write ! (f , ";") ? ; } for (i , subparam) in param . iter () . enumerate () { if i != 0 { write ! (f , ":") ? ; } subparam . fmt (f) ? ; } } write ! (f , "]") } }
    };
}

impl_8!();