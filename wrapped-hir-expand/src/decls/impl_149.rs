macro_rules! deps {
    () => {
        Display!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl fmt :: Display for Display < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut symbol = self . name . symbol . as_str () ; if symbol == "'static" { return f . write_str (symbol) ; } if let Some (s) = symbol . strip_prefix ('\'') { f . write_str ("'") ? ; symbol = s ; } if is_raw_identifier (symbol , self . edition) { f . write_str ("r#") ? ; } f . write_str (symbol) } }
    };
}

impl_149!()