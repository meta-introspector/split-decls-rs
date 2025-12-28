macro_rules! deps {
    () => {
        RawStream!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < T : RawStream + ? Sized > RawStream for & mut T { }
    };
}

impl_44!()