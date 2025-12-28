macro_rules! deps {
    () => {
        Error!();
        ErrorImpl!();
        ContextError!();
        StdError!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < C > StdError for ContextError < C , Error > where C : Display , { fn source (& self) -> Option < & (dyn StdError + 'static) > { Some (unsafe { crate :: ErrorImpl :: error (self . error . inner . by_ref ()) }) } # [cfg (error_generic_member_access)] fn provide < 'a > (& 'a self , request : & mut Request < 'a >) { Error :: provide (& self . error , request) ; } }
    };
}

impl_28!()