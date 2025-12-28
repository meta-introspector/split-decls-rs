macro_rules! deps {
    () => {
        Result!();
        HRESULT!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl < T > From < Result < T > > for HRESULT { fn from (result : Result < T >) -> Self { if let Err (error) = result { return error . into () ; } Self (0) } }
    };
}

impl_84!();