macro_rules! deps {
    () => {
        Mmap!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl AsRef < [u8] > for Mmap { # [inline] fn as_ref (& self) -> & [u8] { self . deref () } }
    };
}

impl_17!()