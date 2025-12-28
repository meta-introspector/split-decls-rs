macro_rules! deps {
    () => {
        CppInterface!();
    };
}

macro_rules! impl_270 {
    () => {
        deps!();
        impl Ord for CppInterface { fn cmp (& self , other : & Self) -> Ordering { self . def . name () . cmp (other . def . name ()) } }
    };
}

impl_270!();