macro_rules! deps {
    () => {
        Value!();
        Map!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl Hash for Map < String , Value > { fn hash < H : Hasher > (& self , state : & mut H) { # [cfg (not (feature = "preserve_order"))] { self . map . hash (state) ; } # [cfg (feature = "preserve_order")] { let mut kv = Vec :: from_iter (& self . map) ; kv . sort_unstable_by (| a , b | a . 0 . cmp (b . 0)) ; kv . hash (state) ; } } }
    };
}

impl_84!();