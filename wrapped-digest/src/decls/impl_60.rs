macro_rules! deps {
    () => {
        CtOutput!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl < T : OutputSizeUser > From < Output < T > > for CtOutput < T > { # [inline (always)] fn from (bytes : Output < T >) -> Self { Self { bytes } } }
    };
}

impl_60!();