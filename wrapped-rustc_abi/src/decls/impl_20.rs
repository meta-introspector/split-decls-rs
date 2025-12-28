macro_rules! deps {
    () => {
        ExternAbi!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl PartialEq for ExternAbi { fn eq (& self , rhs : & Self) -> bool { self . cmp (rhs) == Ordering :: Equal } }
    };
}

impl_20!()