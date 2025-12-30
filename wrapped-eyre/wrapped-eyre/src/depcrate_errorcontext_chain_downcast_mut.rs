// Generated macro for context_chain_downcast_mut (function)
macro_rules! Depcrate_errorcontext_chain_downcast_mut {
() => {
// Module: crate::error
// Provides: {"context_chain_downcast_mut"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = ""] # [doc = " Requires layout of *e to match ErrorImpl<ContextError<D, Report>>."] unsafe fn context_chain_downcast_mut < D > (e : MutPtr < '_ , ErrorImpl < () > > , target : TypeId ,) -> Option < NonNull < () > > where D : 'static , { let unerased = unsafe { e . cast :: < ErrorImpl < ContextError < D , Report > > > () . into_mut () } ; if TypeId :: of :: < D > () == target { let addr = NonNull :: from (& unerased . _object . msg) . cast :: < () > () ; Some (addr) } else { let source = & mut unerased . _object . error ; unsafe { (source . vtable () . object_downcast_mut) (source . inner . as_mut () , target) } } }
};
}
