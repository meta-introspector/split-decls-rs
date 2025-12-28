macro_rules! impl_820 {
    () => {
        impl < T > Ord for OrderWrapper < T > { fn cmp (& self , other : & Self) -> Ordering { other . index . cmp (& self . index) } }
    };
}

impl_820!();