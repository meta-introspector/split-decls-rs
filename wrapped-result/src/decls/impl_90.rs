macro_rules! deps {
    () => {
        BOOL!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl From < BOOL > for bool { fn from (value : BOOL) -> Self { value . as_bool () } }
    };
}

impl_90!()