macro_rules! deps {
    () => {
        Result!();
        TomlLockfilePackageId!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl fmt :: Display for TomlLockfilePackageId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , self . name) ? ; if let Some (s) = & self . version { write ! (f , " {}" , s) ? ; } if let Some (s) = & self . source { write ! (f , " ({})" , s . as_url ()) ? ; } Ok (()) } }
    };
}

impl_58!();