macro_rules! deps {
    () => {
        Literal!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl fmt :: Display for Literal { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . negate { write ! (f , "not(") ? ; } match & self . var { Some (var) => var . fmt (f) ? , None => f . write_str ("<invalid>") ? , } if self . negate { f . write_char (')') ? ; } Ok (()) } }
    };
}

impl_18!();