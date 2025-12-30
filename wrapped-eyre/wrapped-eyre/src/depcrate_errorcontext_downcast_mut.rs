// Generated macro for context_downcast_mut (function)
macro_rules! Depcrate_errorcontext_downcast_mut {
() => {
// Module: crate::error
// Provides: {"context_downcast_mut"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = ""] # [doc = " Requires layout of *e to match ErrorImpl<ContextError<D, E>>."] unsafe fn context_downcast_mut < D , E > (e : MutPtr < '_ , ErrorImpl < () > > , target : TypeId ,) -> Option < NonNull < () > > where D : 'static , E : 'static , { if TypeId :: of :: < D > () == target { let unerased = unsafe { e . cast :: < ErrorImpl < ContextError < D , E > > > () . into_mut () } ; let addr = NonNull :: from (& unerased . _object . msg) . cast :: < () > () ; Some (addr) } else if TypeId :: of :: < E > () == target { let unerased = unsafe { e . cast :: < ErrorImpl < ContextError < D , E > > > () . into_mut () } ; let addr = NonNull :: from (& mut unerased . _object . error) . cast :: < () > () ; Some (addr) } else { None } }
};
}
