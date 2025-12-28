macro_rules! deps {
    () => {
        CtOutput!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < 'a , T : OutputSizeUser > From < & 'a Output < T > > for CtOutput < T > { # [inline (always)] fn from (bytes : & 'a Output < T >) -> Self { bytes . clone () . into () } }
    };
}

impl_8!();