macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        impl From < & '_ Utf8Path > for Box < Path > { fn from (path : & Utf8Path) -> Box < Path > { AsRef :: < Path > :: as_ref (path) . into () } }
    };
}

impl_82!();