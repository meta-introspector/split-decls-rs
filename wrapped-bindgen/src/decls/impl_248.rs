macro_rules! deps {
    () => {
        CppDelegate!();
    };
}

macro_rules! impl_248 {
    () => {
        deps!();
        impl Ord for CppDelegate { fn cmp (& self , other : & Self) -> Ordering { (self . def . name () , self . def) . cmp (& (other . def . name () , other . def)) } }
    };
}

impl_248!();