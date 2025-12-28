macro_rules! deps {
    () => {
        AnyValueId!();
    };
}

macro_rules! impl_608 {
    () => {
        deps!();
        impl Ord for AnyValueId { fn cmp (& self , other : & Self) -> std :: cmp :: Ordering { self . type_id . cmp (& other . type_id) } }
    };
}

impl_608!();