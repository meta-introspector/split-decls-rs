macro_rules! deps {
    () => {
        ReferenceExt!();
        Reference!();
        Repository!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl ReferenceExt for gix_ref :: Reference { fn attach (self , repo : & crate :: Repository) -> crate :: Reference < '_ > { crate :: Reference :: from_ref (self , repo) } }
    };
}

impl_20!();