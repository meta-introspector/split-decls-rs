macro_rules! deps {
    () => {
        ContextError!();
        Mut!();
        ErrorImpl!();
    };
}

macro_rules! context_downcast_mut {
    () => {
        deps!();
        # [cfg (all (feature = "std" , anyhow_no_ptr_addr_of))] unsafe fn context_downcast_mut < C , E > (e : Mut < ErrorImpl > , target : TypeId) -> Option < Mut < () > > where C : 'static , E : 'static , { if TypeId :: of :: < C > () == target { let unerased_mut = e . cast :: < ErrorImpl < ContextError < C , E > > > () ; let unerased = unsafe { unerased_mut . deref_mut () } ; Some (Mut :: new (& mut unerased . _object . context) . cast :: < () > ()) } else if TypeId :: of :: < E > () == target { let unerased_mut = e . cast :: < ErrorImpl < ContextError < C , E > > > () ; let unerased = unsafe { unerased_mut . deref_mut () } ; Some (Mut :: new (& mut unerased . _object . error) . cast :: < () > ()) } else { None } }
    };
}

context_downcast_mut!();