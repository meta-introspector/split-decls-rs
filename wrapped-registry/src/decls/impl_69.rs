macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl AsRef < [u8] > for Value { fn as_ref (& self) -> & [u8] { & self . data } }
    };
}

impl_69!()