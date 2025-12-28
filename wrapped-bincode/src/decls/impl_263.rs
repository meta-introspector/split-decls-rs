macro_rules! deps {
    () => {
        Varint!();
        IntEncoding!();
    };
}

macro_rules! impl_263 {
    () => {
        deps!();
        impl InternalIntEncodingConfig for Varint { const INT_ENCODING : IntEncoding = IntEncoding :: Variable ; }
    };
}

impl_263!()