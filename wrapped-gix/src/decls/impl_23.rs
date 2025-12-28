macro_rules! deps {
    () => {
        Spec!();
        RevSpecExt!();
        Repository!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl RevSpecExt for gix_revision :: Spec { fn attach (self , repo : & crate :: Repository) -> crate :: revision :: Spec < '_ > { crate :: revision :: Spec { inner : self , path : None , first_ref : None , second_ref : None , repo , } } }
    };
}

impl_23!();