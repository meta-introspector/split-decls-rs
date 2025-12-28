macro_rules! deps {
    () => {
        CppStruct!();
    };
}

macro_rules! impl_286 {
    () => {
        deps!();
        impl Ord for CppStruct { fn cmp (& self , other : & Self) -> Ordering { (self . name , self . def) . cmp (& (other . name , other . def)) } }
    };
}

impl_286!()