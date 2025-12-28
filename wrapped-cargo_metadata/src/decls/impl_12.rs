macro_rules! deps {
    () => {
        Diagnostic!();
        Result!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl fmt :: Display for Diagnostic { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { if let Some (ref rendered) = self . rendered { f . write_str (rendered) ? ; } else { f . write_str ("cargo didn't render this message") ? ; } Ok (()) } }
    };
}

impl_12!();