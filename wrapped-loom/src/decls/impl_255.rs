macro_rules! deps {
    () => {
        AtomicBool!();
    };
}

macro_rules! impl_255 {
    () => {
        deps!();
        impl From < bool > for AtomicBool { fn from (b : bool) -> Self { Self :: new (b) } }
    };
}

impl_255!();