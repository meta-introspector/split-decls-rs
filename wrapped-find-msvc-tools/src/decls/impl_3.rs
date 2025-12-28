macro_rules! deps {
    () => {
        Env!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl AsRef < OsStr > for Env { fn as_ref (& self) -> & OsStr { self . deref () } }
    };
}

impl_3!()