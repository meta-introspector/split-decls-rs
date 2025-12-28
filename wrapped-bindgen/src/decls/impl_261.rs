macro_rules! deps {
    () => {
        CppFn!();
    };
}

macro_rules! impl_261 {
    () => {
        deps!();
        impl Ord for CppFn { fn cmp (& self , other : & Self) -> Ordering { (self . method . name () , self . method) . cmp (& (other . method . name () , other . method)) } }
    };
}

impl_261!()