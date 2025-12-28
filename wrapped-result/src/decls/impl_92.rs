macro_rules! deps {
    () => {
        BOOL!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl From < bool > for BOOL { fn from (value : bool) -> Self { if value { Self (1) } else { Self (0) } } }
    };
}

impl_92!();