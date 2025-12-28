macro_rules! deps {
    () => {
        Chain!();
        Ref!();
        ErrorImpl!();
        StdError!();
        Mut!();
        Backtrace!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl ErrorImpl { pub (crate) unsafe fn error (this : Ref < Self >) -> & (dyn StdError + Send + Sync + 'static) { unsafe { (vtable (this . ptr) . object_ref) (this) . deref () } } # [cfg (any (feature = "std" , not (anyhow_no_core_error)))] pub (crate) unsafe fn error_mut (this : Mut < Self >) -> & mut (dyn StdError + Send + Sync + 'static) { # [cfg (not (anyhow_no_ptr_addr_of))] return unsafe { (vtable (this . ptr) . object_ref) (this . by_ref ()) . by_mut () . deref_mut () } ; # [cfg (anyhow_no_ptr_addr_of)] return unsafe { (vtable (this . ptr) . object_mut) (this) } ; } # [cfg (any (std_backtrace , feature = "backtrace"))] pub (crate) unsafe fn backtrace (this : Ref < Self >) -> & Backtrace { unsafe { this . deref () } . backtrace . as_ref () . or_else (| | { # [cfg (error_generic_member_access)] return nightly :: request_ref_backtrace (unsafe { Self :: error (this) }) ; # [cfg (not (error_generic_member_access))] return unsafe { (vtable (this . ptr) . object_backtrace) (this) } ; }) . expect ("backtrace capture failed") } # [cfg (error_generic_member_access)] unsafe fn provide < 'a > (this : Ref < 'a , Self > , request : & mut Request < 'a >) { if let Some (backtrace) = unsafe { & this . deref () . backtrace } { nightly :: provide_ref_backtrace (request , backtrace) ; } nightly :: provide (unsafe { Self :: error (this) } , request) ; } # [cold] pub (crate) unsafe fn chain (this : Ref < Self >) -> Chain { Chain :: new (unsafe { Self :: error (this) }) } }
    };
}

impl_74!();