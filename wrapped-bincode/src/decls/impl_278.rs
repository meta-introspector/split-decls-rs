macro_rules! deps {
    () => {
        WithContext!();
        Sealed!();
        Decoder!();
    };
}

macro_rules! impl_278 {
    () => {
        deps!();
        impl < C , D : Decoder + ? Sized > Sealed for WithContext < '_ , D , C > { }
    };
}

impl_278!();