macro_rules! deps {
    () => {
        Decoder!();
        Sealed!();
        WithContext!();
    };
}

macro_rules! impl_278 {
    () => {
        deps!();
        impl < C , D : Decoder + ? Sized > Sealed for WithContext < '_ , D , C > { }
    };
}

impl_278!()