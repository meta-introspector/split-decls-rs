macro_rules! deps {
    () => {
        GUID!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        impl From < u128 > for GUID { fn from (value : u128) -> Self { Self :: from_u128 (value) } }
    };
}

impl_137!()