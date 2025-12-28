macro_rules! deps {
    () => {
        TypeName!();
    };
}

macro_rules! impl_227 {
    () => {
        deps!();
        impl Ord for TypeName { fn cmp (& self , other : & Self) -> Ordering { (self . 1 , self . 0) . cmp (& (other . 1 , other . 0)) } }
    };
}

impl_227!()