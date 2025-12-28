macro_rules! deps {
    () => {
        CppEnum!();
    };
}

macro_rules! impl_256 {
    () => {
        deps!();
        impl PartialOrd for CppEnum { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_256!();