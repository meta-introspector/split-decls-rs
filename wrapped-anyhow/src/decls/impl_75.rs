macro_rules! deps {
    () => {
        StdError!();
        ErrorImpl!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl < E > StdError for ErrorImpl < E > where E : StdError , { fn source (& self) -> Option < & (dyn StdError + 'static) > { unsafe { ErrorImpl :: error (self . erase ()) . source () } } # [cfg (error_generic_member_access)] fn provide < 'a > (& 'a self , request : & mut Request < 'a >) { unsafe { ErrorImpl :: provide (self . erase () , request) } } }
    };
}

impl_75!();