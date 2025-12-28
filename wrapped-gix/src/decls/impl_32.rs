macro_rules! deps {
    () => {
        Entry!();
        Repository!();
        TreeEntryExt!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl TreeEntryExt for gix_object :: tree :: Entry { fn attach (self , repo : & crate :: Repository) -> crate :: object :: tree :: Entry < '_ > { crate :: object :: tree :: Entry { inner : self , repo } } }
    };
}

impl_32!();