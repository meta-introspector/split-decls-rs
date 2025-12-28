macro_rules! deps {
    () => {
        TypeName!();
    };
}

macro_rules! impl_228 {
    () => {
        deps!();
        impl PartialOrd for TypeName { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_228!();