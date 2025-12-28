macro_rules! deps {
    () => {
        RelPathBuf!();
        RelPath!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl ops :: Deref for RelPathBuf { type Target = RelPath ; fn deref (& self) -> & RelPath { self . as_path () } }
    };
}

impl_25!()