macro_rules! deps {
    () => {
        OsStr!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl From < & '_ OsStr > for OsStr { fn from (id : & '_ OsStr) -> Self { id . clone () } }
    };
}

impl_106!();