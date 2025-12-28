macro_rules! deps {
    () => {
        Str!();
    };
}

macro_rules! impl_207 {
    () => {
        deps!();
        impl AsRef < str > for Str { # [inline] fn as_ref (& self) -> & str { self . as_str () } }
    };
}

impl_207!()