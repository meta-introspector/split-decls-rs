// Generated macro for context_chain_drop_rest (function)
macro_rules! Depcrate_errorcontext_chain_drop_rest {
() => {
// Module: crate::error
// Provides: {"context_chain_drop_rest"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = ""] # [doc = " Requires layout of *e to match ErrorImpl<ContextError<D, Report>>."] unsafe fn context_chain_drop_rest < D > (e : OwnedPtr < ErrorImpl < () > > , target : TypeId) where D : 'static , { if TypeId :: of :: < D > () == target { let unerased = unsafe { e . cast :: < ErrorImpl < ContextError < ManuallyDrop < D > , Report > > > () . into_box () } ; drop (unerased) ; } else { unsafe { let unerased = e . cast :: < ErrorImpl < ContextError < D , ManuallyDrop < Report > > > > () . into_box () ; let inner = ptr :: read (& unerased . as_ref () . _object . error . inner) ; drop (unerased) ; (header (inner . as_ref ()) . vtable . object_drop_rest) (inner , target) ; } } }
};
}
