macro_rules! deps {
    () => {
        Proxy!();
    };
}

macro_rules! impl_155 {
    () => {
        deps!();
        impl < T > gix_object :: Exists for Proxy < T > where T : gix_object :: Exists , { fn exists (& self , id : & gix_hash :: oid) -> bool { self . memory . as_ref () . is_some_and (| map | map . borrow () . contains_key (id)) || self . inner . exists (id) } }
    };
}

impl_155!();