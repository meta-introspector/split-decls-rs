macro_rules! deps {
    () => {
        Constness!();
    };
}

macro_rules! impl_317 {
    () => {
        deps!();
        impl fmt :: Display for Constness { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match * self { Self :: Const => "const" , Self :: NotConst => "non-const" , }) } }
    };
}

impl_317!();