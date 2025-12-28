macro_rules! deps {
    () => {
        Engine!();
        DecoderReaderEngine!();
    };
}

macro_rules! impl_187 {
    () => {
        deps!();
        impl < E : Engine > From < E > for DecoderReaderEngine < E > { fn from (value : E) -> Self { Self { engine : value } } }
    };
}

impl_187!()