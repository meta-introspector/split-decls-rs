macro_rules! deps {
    () => {
        AbsPathBuf!();
        AbsPath!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl ops :: Deref for AbsPathBuf { type Target = AbsPath ; fn deref (& self) -> & AbsPath { self . as_path () } }
    };
}

impl_3!()