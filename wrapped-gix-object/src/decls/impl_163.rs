macro_rules! deps {
    () => {
        Never!();
        Exists!();
    };
}

macro_rules! impl_163 {
    () => {
        deps!();
        impl super :: Exists for Never { fn exists (& self , _id : & gix_hash :: oid) -> bool { false } }
    };
}

impl_163!();