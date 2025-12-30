// Generated macro for impl_212 (impl)
macro_rules! Depcrate_internalimpl_212 {
() => {
// Module: crate::internal
// Provides: {"impl_212"}
// Dependencies: {}
impl IsStreaming for Streaming { fn incomplete < E , F : FnOnce () -> E > (needed : Needed , _err_f : F) -> Err < E > { Err :: Incomplete (needed) } # [inline] fn is_streaming () -> bool { true } }
};
}
