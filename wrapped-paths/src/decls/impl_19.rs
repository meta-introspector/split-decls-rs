macro_rules! deps {
    () => {
        AbsPath!();
        AbsPathBuf!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl ToOwned for AbsPath { type Owned = AbsPathBuf ; fn to_owned (& self) -> Self :: Owned { AbsPathBuf (self . 0 . to_owned ()) } }
    };
}

impl_19!()