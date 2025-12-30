// Generated macro for context_chain_downcast (function)
macro_rules! Depcrate_errorcontext_chain_downcast {
() => {
// Module: crate::error
// Provides: {"context_chain_downcast"}
// Dependencies: {}
unsafe fn context_chain_downcast < C > (e : Ref < ErrorImpl > , target : TypeId) -> Option < Ref < () > > where C : 'static , { let unerased_ref = e . cast :: < ErrorImpl < ContextError < C , Error > > > () ; let unerased = unsafe { unerased_ref . deref () } ; if TypeId :: of :: < C > () == target { Some (Ref :: new (& unerased . _object . context) . cast :: < () > ()) } else { let source = & unerased . _object . error ; unsafe { (vtable (source . inner . ptr) . object_downcast) (source . inner . by_ref () , target) } } }
};
}
