macro_rules! deps {
    () => {
        AbsPath!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < P : AsRef < Path > + ? Sized > PartialEq < P > for AbsPath { fn eq (& self , other : & P) -> bool { self . 0 . as_std_path () == other . as_ref () } }
    };
}

impl_15!()