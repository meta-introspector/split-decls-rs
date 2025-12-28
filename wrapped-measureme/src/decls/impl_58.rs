macro_rules! deps {
    () => {
        Addr!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl Addr { pub fn as_usize (self) -> usize { self . 0 as usize } }
    };
}

impl_58!()