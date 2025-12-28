macro_rules! deps {
    () => {
        Local!();
    };
}

macro_rules! impl_353 {
    () => {
        deps!();
        impl Ord for Local { fn cmp (& self , other : & Self) -> std :: cmp :: Ordering { self . binding_id . cmp (& other . binding_id) } }
    };
}

impl_353!();