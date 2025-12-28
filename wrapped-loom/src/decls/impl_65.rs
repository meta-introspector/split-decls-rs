macro_rules! deps {
    () => {
        Execution!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl fmt :: Debug for Execution { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_struct ("Execution") . field ("path" , & self . path) . field ("threads" , & self . threads) . finish () } }
    };
}

impl_65!();