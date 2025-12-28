macro_rules! deps {
    () => {
        CppConst!();
    };
}

macro_rules! impl_241 {
    () => {
        deps!();
        impl PartialOrd for CppConst { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_241!();