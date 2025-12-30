// Generated macro for object_boxed (function)
macro_rules! Depcrate_errorobject_boxed {
() => {
// Module: crate::error
// Provides: {"object_boxed"}
// Dependencies: {}
# [cfg (any (feature = "std" , not (anyhow_no_core_error)))] unsafe fn object_boxed < E > (e : Own < ErrorImpl >) -> Box < dyn StdError + Send + Sync + 'static > where E : StdError + Send + Sync + 'static , { let unerased_own = e . cast :: < ErrorImpl < E > > () ; unsafe { unerased_own . boxed () } }
};
}
