macro_rules! deps {
    () => {
        OsStr!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl From < & '_ & 'static str > for OsStr { fn from (name : & '_ & 'static str) -> Self { Self :: from_static_ref ((* name) . as_ref ()) } }
    };
}

impl_117!()