macro_rules! deps {
    () => {
        Statement!();
        Result!();
    };
}

macro_rules! impl_276 {
    () => {
        deps!();
        impl fmt :: Debug for Statement < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let sql = if self . stmt . is_null () { Ok ("") } else { self . stmt . sql () . unwrap () . to_str () } ; f . debug_struct ("Statement") . field ("conn" , self . conn) . field ("stmt" , & self . stmt) . field ("sql" , & sql) . finish () } }
    };
}

impl_276!();