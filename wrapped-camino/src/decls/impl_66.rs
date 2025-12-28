macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl From < & '_ Utf8Path > for Arc < Path > { fn from (path : & Utf8Path) -> Arc < Path > { AsRef :: < Path > :: as_ref (path) . into () } }
    };
}

impl_66!()