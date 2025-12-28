macro_rules! deps {
    () => {
        Str!();
    };
}

macro_rules! impl_201 {
    () => {
        deps!();
        impl From < Str > for Vec < u8 > { fn from (name : Str) -> Self { String :: from (name) . into () } }
    };
}

impl_201!();