macro_rules! deps {
    () => {
        Repository!();
    };
}

macro_rules! impl_338 {
    () => {
        deps!();
        impl gix_object :: Exists for crate :: Repository { fn exists (& self , id : & gix_hash :: oid) -> bool { if id == ObjectId :: empty_tree (self . object_hash ()) { return true ; } self . objects . exists (id) } }
    };
}

impl_338!()