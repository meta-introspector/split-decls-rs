macro_rules! deps {
    () => {
        CppInterface!();
    };
}

macro_rules! impl_271 {
    () => {
        deps!();
        impl PartialOrd for CppInterface { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_271!()