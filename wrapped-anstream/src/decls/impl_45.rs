macro_rules! deps {
    () => {
        RawStream!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < T : RawStream + ? Sized > RawStream for Box < T > { }
    };
}

impl_45!();