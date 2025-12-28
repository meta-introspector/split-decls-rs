macro_rules! deps {
    () => {
        Result!();
        Error!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl Debug for Error { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { let error = self . as_serde_de_error :: < serde :: de :: value :: Error > () ; Debug :: fmt (& error , formatter) } }
    };
}

impl_58!()