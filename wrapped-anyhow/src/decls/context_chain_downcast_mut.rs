macro_rules! deps {
    () => {
        ErrorImpl!();
        Mut!();
        ContextError!();
        Error!();
    };
}

macro_rules! context_chain_downcast_mut {
    () => {
        deps!();
        # [cfg (anyhow_no_ptr_addr_of)] unsafe fn context_chain_downcast_mut < C > (e : Mut < ErrorImpl > , target : TypeId) -> Option < Mut < () > > where C : 'static , { let unerased_mut = e . cast :: < ErrorImpl < ContextError < C , Error > > > () ; let unerased = unsafe { unerased_mut . deref_mut () } ; if TypeId :: of :: < C > () == target { Some (Mut :: new (& mut unerased . _object . context) . cast :: < () > ()) } else { let source = & mut unerased . _object . error ; unsafe { (vtable (source . inner . ptr) . object_downcast_mut) (source . inner . by_mut () , target) } } }
    };
}

context_chain_downcast_mut!();