macro_rules! deps {
    () => {
        Possibilities!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl std :: fmt :: Debug for Possibilities { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { let mut l = f . debug_set () ; if (self . 0 & P_SCHEME) != 0 { l . entry (& "scheme") ; } if (self . 0 & P_PARAM_KEY) != 0 { l . entry (& "param_key") ; } if (self . 0 & P_EOF) != 0 { l . entry (& "eof") ; } if (self . 0 & P_WHITESPACE) != 0 { l . entry (& "whitespace") ; } if (self . 0 & P_COMMA_PARAM_KEY) != 0 { l . entry (& "comma_param_key") ; } if (self . 0 & P_COMMA_EOF) != 0 { l . entry (& "comma_eof") ; } l . finish () } }
    };
}

impl_14!()