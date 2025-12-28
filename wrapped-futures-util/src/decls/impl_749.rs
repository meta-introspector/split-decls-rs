macro_rules! deps {
    () => {
        Repeat!();
    };
}

macro_rules! impl_749 {
    () => {
        deps!();
        impl < T > Unpin for Repeat < T > { }
    };
}

impl_749!()