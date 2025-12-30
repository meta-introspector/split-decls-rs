// Generated macro for context_drop_rest (function)
macro_rules! Depcrate_errorcontext_drop_rest {
() => {
// Module: crate::error
// Provides: {"context_drop_rest"}
// Dependencies: {}
# [cfg (any (feature = "std" , not (anyhow_no_core_error)))] unsafe fn context_drop_rest < C , E > (e : Own < ErrorImpl > , target : TypeId) where C : 'static , E : 'static , { if TypeId :: of :: < C > () == target { let unerased_own = e . cast :: < ErrorImpl < ContextError < ManuallyDrop < C > , E > > > () ; drop (unsafe { unerased_own . boxed () }) ; } else { let unerased_own = e . cast :: < ErrorImpl < ContextError < C , ManuallyDrop < E > > > > () ; drop (unsafe { unerased_own . boxed () }) ; } }
};
}
