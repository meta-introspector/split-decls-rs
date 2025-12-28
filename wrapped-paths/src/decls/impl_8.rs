macro_rules! deps {
    () => {
        AbsPathBuf!();
        AbsPath!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl Borrow < AbsPath > for AbsPathBuf { fn borrow (& self) -> & AbsPath { self . as_path () } }
    };
}

impl_8!()