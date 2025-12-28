macro_rules! deps {
    () => {
        Header!();
        Never!();
        Error!();
    };
}

macro_rules! impl_161 {
    () => {
        deps!();
        impl super :: FindHeader for Never { fn try_header (& self , _id : & gix_hash :: oid) -> Result < Option < crate :: Header > , Error > { Ok (None) } }
    };
}

impl_161!();