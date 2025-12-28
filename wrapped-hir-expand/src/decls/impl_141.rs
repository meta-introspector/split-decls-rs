macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! impl_141 {
    () => {
        deps!();
        impl Ord for Name { fn cmp (& self , other : & Self) -> std :: cmp :: Ordering { self . symbol . as_str () . cmp (other . symbol . as_str ()) } }
    };
}

impl_141!();