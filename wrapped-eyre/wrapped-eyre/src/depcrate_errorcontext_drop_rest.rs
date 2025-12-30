// Generated macro for context_drop_rest (function)
macro_rules! Depcrate_errorcontext_drop_rest {
() => {
// Module: crate::error
// Provides: {"context_drop_rest"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = ""] # [doc = " Requires layout of *e to match ErrorImpl<ContextError<D, E>>."] unsafe fn context_drop_rest < D , E > (e : OwnedPtr < ErrorImpl < () > > , target : TypeId) where D : 'static , E : 'static , { if TypeId :: of :: < D > () == target { unsafe { e . cast :: < ErrorImpl < ContextError < ManuallyDrop < D > , E > > > () . into_box () } ; } else { debug_assert_eq ! (TypeId :: of ::< E > () , target) ; unsafe { e . cast :: < ErrorImpl < ContextError < D , ManuallyDrop < E > > > > () . into_box () } ; } }
};
}
