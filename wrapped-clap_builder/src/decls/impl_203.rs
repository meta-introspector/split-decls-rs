macro_rules! deps {
    () => {
        Str!();
    };
}

macro_rules! impl_203 {
    () => {
        deps!();
        impl From < Str > for std :: path :: PathBuf { fn from (name : Str) -> Self { String :: from (name) . into () } }
    };
}

impl_203!();