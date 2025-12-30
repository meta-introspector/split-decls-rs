// Generated macro for context_downcast (function)
macro_rules! Depcrate_errorcontext_downcast {
() => {
// Module: crate::error
// Provides: {"context_downcast"}
// Dependencies: {}
# [cfg (any (feature = "std" , not (anyhow_no_core_error)))] unsafe fn context_downcast < C , E > (e : Ref < ErrorImpl > , target : TypeId) -> Option < Ref < () > > where C : 'static , E : 'static , { if TypeId :: of :: < C > () == target { let unerased_ref = e . cast :: < ErrorImpl < ContextError < C , E > > > () ; let unerased = unsafe { unerased_ref . deref () } ; Some (Ref :: new (& unerased . _object . context) . cast :: < () > ()) } else if TypeId :: of :: < E > () == target { let unerased_ref = e . cast :: < ErrorImpl < ContextError < C , E > > > () ; let unerased = unsafe { unerased_ref . deref () } ; Some (Ref :: new (& unerased . _object . error) . cast :: < () > ()) } else { None } }
};
}
