macro_rules! deps {
    () => {
        GUID!();
    };
}

macro_rules! impl_138 {
    () => {
        deps!();
        impl From < GUID > for u128 { fn from (value : GUID) -> Self { value . to_u128 () } }
    };
}

impl_138!()