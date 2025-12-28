macro_rules! impl_141 {
    () => {
        impl Ord for Inner { fn cmp (& self , other : & Inner) -> std :: cmp :: Ordering { self . as_os_str () . cmp (other . as_os_str ()) } }
    };
}

impl_141!()