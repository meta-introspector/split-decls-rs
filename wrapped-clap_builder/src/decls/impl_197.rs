macro_rules! deps {
    () => {
        Str!();
    };
}

macro_rules! impl_197 {
    () => {
        deps!();
        impl From < & 'static str > for Str { fn from (name : & 'static str) -> Self { Self :: from_static_ref (name) } }
    };
}

impl_197!();