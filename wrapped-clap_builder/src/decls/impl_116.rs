macro_rules! deps {
    () => {
        OsStr!();
    };
}

macro_rules! impl_116 {
    () => {
        deps!();
        impl From < & 'static str > for OsStr { fn from (name : & 'static str) -> Self { Self :: from_static_ref (name . as_ref ()) } }
    };
}

impl_116!()