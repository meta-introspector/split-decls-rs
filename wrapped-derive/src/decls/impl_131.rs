macro_rules! deps {
    () => {
        UuidVersionValidation!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl FromMeta for UuidVersionValidation { fn from_word () -> darling :: Result < Self > { Ok (UuidVersionValidation :: None) } fn from_value (value : & Lit) -> darling :: Result < Self > { Ok (UuidVersionValidation :: Value (value . clone ())) } }
    };
}

impl_131!()