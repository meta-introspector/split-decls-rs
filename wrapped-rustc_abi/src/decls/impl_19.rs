macro_rules! deps {
    () => {
        ExternAbi!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl PartialOrd for ExternAbi { fn partial_cmp (& self , rhs : & Self) -> Option < Ordering > { Some (self . cmp (rhs)) } }
    };
}

impl_19!();