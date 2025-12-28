macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl StdError for Error { fn source (& self) -> Option < & (dyn StdError + 'static) > { self . inner . cause . as_ref () . map (| cause | & * * cause as & (dyn StdError + 'static)) } }
    };
}

impl_112!()