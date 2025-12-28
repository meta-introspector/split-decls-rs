macro_rules! deps {
    () => {
        CtOutput!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl < T : OutputSizeUser > From < Output < T > > for CtOutput < T > { # [inline (always)] fn from (bytes : Output < T >) -> Self { Self { bytes } } }
    };
}

impl_7!();