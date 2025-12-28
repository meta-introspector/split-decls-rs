macro_rules! deps {
    () => {
        RawStream!();
        Stderr!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl RawStream for std :: io :: Stderr { }
    };
}

impl_48!()