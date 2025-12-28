macro_rules! deps {
    () => {
        RawStream!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl RawStream for std :: fs :: File { }
    };
}

impl_54!()