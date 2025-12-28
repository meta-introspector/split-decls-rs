macro_rules! deps {
    () => {
        Type!();
    };
}

macro_rules! impl_316 {
    () => {
        deps!();
        impl PartialOrd for Type { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_316!();