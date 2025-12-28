macro_rules! deps {
    () => {
        AbsPathBuf!();
        AbsPath!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl AsRef < AbsPath > for AbsPathBuf { fn as_ref (& self) -> & AbsPath { self . as_path () } }
    };
}

impl_7!()