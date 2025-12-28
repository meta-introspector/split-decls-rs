macro_rules! deps {
    () => {
        ErrorImpl!();
        ContextError!();
        Error!();
        Ref!();
    };
}

macro_rules! context_chain_downcast {
    () => {
        deps!();
        unsafe fn context_chain_downcast < C > (e : Ref < ErrorImpl > , target : TypeId) -> Option < Ref < () > > where C : 'static , { let unerased_ref = e . cast :: < ErrorImpl < ContextError < C , Error > > > () ; let unerased = unsafe { unerased_ref . deref () } ; if TypeId :: of :: < C > () == target { Some (Ref :: new (& unerased . _object . context) . cast :: < () > ()) } else { let source = & unerased . _object . error ; unsafe { (vtable (source . inner . ptr) . object_downcast) (source . inner . by_ref () , target) } } }
    };
}

context_chain_downcast!()