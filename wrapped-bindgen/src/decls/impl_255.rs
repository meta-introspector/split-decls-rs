macro_rules! deps {
    () => {
        CppEnum!();
    };
}

macro_rules! impl_255 {
    () => {
        deps!();
        impl Ord for CppEnum { fn cmp (& self , other : & Self) -> Ordering { self . def . name () . cmp (other . def . name ()) } }
    };
}

impl_255!()