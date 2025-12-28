macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl StdError for Error { fn source (& self) -> Option < & (dyn StdError + 'static) > { self . inner . source . as_ref () . map (| e | & * * e as _) } }
    };
}

impl_11!();