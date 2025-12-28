macro_rules! impl_227 {
    () => {
        impl Ord for Inner { fn cmp (& self , other : & Inner) -> std :: cmp :: Ordering { self . as_str () . cmp (other . as_str ()) } }
    };
}

impl_227!();