macro_rules! deps {
    () => {
        Str!();
    };
}

macro_rules! impl_198 {
    () => {
        deps!();
        impl From < & '_ & 'static str > for Str { fn from (name : & '_ & 'static str) -> Self { Self :: from_static_ref (name) } }
    };
}

impl_198!()