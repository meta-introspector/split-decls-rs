macro_rules! deps {
    () => {
        ReasonPhrase!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl AsRef < [u8] > for ReasonPhrase { fn as_ref (& self) -> & [u8] { & self . 0 } }
    };
}

impl_131!();