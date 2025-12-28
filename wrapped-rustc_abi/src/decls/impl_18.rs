macro_rules! deps {
    () => {
        ExternAbi!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl Ord for ExternAbi { fn cmp (& self , rhs : & Self) -> Ordering { self . as_str () . cmp (rhs . as_str ()) } }
    };
}

impl_18!();