macro_rules! deps {
    () => {
        Signature!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl AsRef < [u8] > for Signature { fn as_ref (& self) -> & [u8] { & self . 0 } }
    };
}

impl_70!();