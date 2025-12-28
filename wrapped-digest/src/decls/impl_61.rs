macro_rules! deps {
    () => {
        CtOutput!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl < 'a , T : OutputSizeUser > From < & 'a Output < T > > for CtOutput < T > { # [inline (always)] fn from (bytes : & 'a Output < T >) -> Self { bytes . clone () . into () } }
    };
}

impl_61!()