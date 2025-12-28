macro_rules! deps {
    () => {
        Result!();
        TomlTrimPathsValue!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl fmt :: Display for TomlTrimPathsValue { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , self . as_str ()) } }
    };
}

impl_143!();