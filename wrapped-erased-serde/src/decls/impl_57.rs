macro_rules! deps {
    () => {
        Error!();
        Result!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl Display for Error { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { let error = self . as_serde_de_error :: < serde :: de :: value :: Error > () ; Display :: fmt (& error , formatter) } }
    };
}

impl_57!();