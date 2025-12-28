macro_rules! deps {
    () => {
        Stdout!();
        RawStream!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl RawStream for std :: io :: Stdout { }
    };
}

impl_46!()