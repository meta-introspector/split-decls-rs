macro_rules! deps {
    () => {
        ContextError!();
        StdError!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < C , E > StdError for ContextError < C , E > where C : Display , E : StdError + 'static , { fn source (& self) -> Option < & (dyn StdError + 'static) > { Some (& self . error) } # [cfg (error_generic_member_access)] fn provide < 'a > (& 'a self , request : & mut Request < 'a >) { nightly :: provide (& self . error , request) ; } }
    };
}

impl_27!()