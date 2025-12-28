macro_rules! deps {
    () => {
        IntEncoding!();
        Fixint!();
    };
}

macro_rules! impl_261 {
    () => {
        deps!();
        impl InternalIntEncodingConfig for Fixint { const INT_ENCODING : IntEncoding = IntEncoding :: Fixed ; }
    };
}

impl_261!();