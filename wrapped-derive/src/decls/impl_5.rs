macro_rules! deps {
    () => {
        DefaultValue!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl FromMeta for DefaultValue { fn from_word () -> darling :: Result < Self > { Ok (DefaultValue :: Default) } fn from_value (value : & Lit) -> darling :: Result < Self > { Ok (DefaultValue :: Value (value . clone ())) } }
    };
}

impl_5!();