macro_rules! deps {
    () => {
        StdError!();
        BoxedError!();
    };
}

macro_rules! impl_147 {
    () => {
        deps!();
        # [cfg (any (feature = "std" , not (anyhow_no_core_error)))] impl StdError for BoxedError { fn source (& self) -> Option < & (dyn StdError + 'static) > { self . 0 . source () } # [cfg (error_generic_member_access)] fn provide < 'a > (& 'a self , request : & mut Request < 'a >) { nightly :: provide (& * self . 0 , request) ; } }
    };
}

impl_147!()