macro_rules! deps {
    () => {
        StringTypedError!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl StdError for StringTypedError { fn source (& self) -> Option < & (dyn StdError + 'static) > { self . source . as_ref () . map (| err | err as & dyn StdError) } }
    };
}

impl_6!()