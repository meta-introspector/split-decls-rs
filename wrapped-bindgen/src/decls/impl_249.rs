macro_rules! deps {
    () => {
        CppDelegate!();
    };
}

macro_rules! impl_249 {
    () => {
        deps!();
        impl PartialOrd for CppDelegate { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_249!();