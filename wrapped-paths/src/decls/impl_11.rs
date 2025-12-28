macro_rules! deps {
    () => {
        AbsPathBuf!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < P : AsRef < Path > + ? Sized > PartialEq < P > for AbsPathBuf { fn eq (& self , other : & P) -> bool { self . 0 . as_std_path () == other . as_ref () } }
    };
}

impl_11!()