macro_rules! deps {
    () => {
        CppFn!();
    };
}

macro_rules! impl_262 {
    () => {
        deps!();
        impl PartialOrd for CppFn { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_262!()