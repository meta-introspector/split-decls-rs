macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl From < & '_ Utf8Path > for Rc < Path > { fn from (path : & Utf8Path) -> Rc < Path > { AsRef :: < Path > :: as_ref (path) . into () } }
    };
}

impl_84!();