// Generated macro for context_downcast (function)
macro_rules! Depcrate_errorcontext_downcast {
() => {
// Module: crate::error
// Provides: {"context_downcast"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = ""] # [doc = " Requires layout of *e to match ErrorImpl<ContextError<D, E>>."] unsafe fn context_downcast < D , E > (e : RefPtr < '_ , ErrorImpl < () > > , target : TypeId ,) -> Option < NonNull < () > > where D : 'static , E : 'static , { if TypeId :: of :: < D > () == target { let unerased = unsafe { e . cast :: < ErrorImpl < ContextError < D , E > > > () . as_ref () } ; let addr = NonNull :: from (& unerased . _object . msg) . cast :: < () > () ; Some (addr) } else if TypeId :: of :: < E > () == target { let unerased = unsafe { e . cast :: < ErrorImpl < ContextError < D , E > > > () . as_ref () } ; let addr = NonNull :: from (& unerased . _object . error) . cast :: < () > () ; Some (addr) } else { None } }
};
}
