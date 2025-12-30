// Generated macro for impl_214 (impl)
macro_rules! Depcrate_internalimpl_214 {
() => {
// Module: crate::internal
// Provides: {"impl_214"}
// Dependencies: {}
impl IsStreaming for Complete { fn incomplete < E , F : FnOnce () -> E > (_needed : Needed , err_f : F) -> Err < E > { Err :: Error (err_f ()) } # [inline] fn is_streaming () -> bool { false } }
};
}
