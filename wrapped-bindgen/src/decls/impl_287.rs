macro_rules! deps {
    () => {
        CppStruct!();
    };
}

macro_rules! impl_287 {
    () => {
        deps!();
        impl PartialOrd for CppStruct { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_287!()