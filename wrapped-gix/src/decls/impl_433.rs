macro_rules! deps {
    () => {
        OwnedOrStaticAtomicBool!();
    };
}

macro_rules! impl_433 {
    () => {
        deps!();
        impl From < & 'static AtomicBool > for OwnedOrStaticAtomicBool { fn from (value : & 'static AtomicBool) -> Self { OwnedOrStaticAtomicBool :: Shared (value) } }
    };
}

impl_433!();