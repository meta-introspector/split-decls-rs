macro_rules! deps {
    () => {
        RawStream!();
        Stdout!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl RawStream for std :: io :: Stdout { }
    };
}

impl_46!();