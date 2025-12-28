macro_rules! deps {
    () => {
        Unit!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl From < & 'static str > for Unit { fn from (v : & 'static str) -> Self { label (v) } }
    };
}

impl_130!();