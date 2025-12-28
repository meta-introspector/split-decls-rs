macro_rules! deps {
    () => {
        Type!();
    };
}

macro_rules! impl_315 {
    () => {
        deps!();
        impl Ord for Type { fn cmp (& self , other : & Self) -> Ordering { self . sort_key () . cmp (& (other . sort_key ())) } }
    };
}

impl_315!()