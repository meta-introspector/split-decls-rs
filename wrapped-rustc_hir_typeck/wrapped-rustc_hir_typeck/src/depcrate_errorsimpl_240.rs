// Generated macro for impl_240 (impl)
macro_rules! Depcrate_errorsimpl_240 {
() => {
// Module: crate::errors
// Provides: {"impl_240"}
// Dependencies: {}
impl HelpUseLatestEdition { pub (crate) fn new () -> Self { let edition = LATEST_STABLE_EDITION ; if rustc_session :: utils :: was_invoked_from_cargo () { Self :: Cargo { edition } } else { Self :: Standalone { edition } } } }
};
}
