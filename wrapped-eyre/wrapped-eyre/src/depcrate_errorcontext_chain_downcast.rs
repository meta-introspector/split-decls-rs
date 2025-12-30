// Generated macro for context_chain_downcast (function)
macro_rules! Depcrate_errorcontext_chain_downcast {
() => {
// Module: crate::error
// Provides: {"context_chain_downcast"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = ""] # [doc = " Requires layout of *e to match ErrorImpl<ContextError<D, Report>>."] unsafe fn context_chain_downcast < D > (e : RefPtr < '_ , ErrorImpl < () > > , target : TypeId ,) -> Option < NonNull < () > > where D : 'static , { let unerased = unsafe { e . cast :: < ErrorImpl < ContextError < D , Report > > > () . as_ref () } ; if TypeId :: of :: < D > () == target { let addr = NonNull :: from (& unerased . _object . msg) . cast :: < () > () ; Some (addr) } else { let source = & unerased . _object . error ; unsafe { (source . vtable () . object_downcast) (source . inner . as_ref () , target) } } }
};
}
