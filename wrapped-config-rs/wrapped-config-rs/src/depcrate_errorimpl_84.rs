// Generated macro for impl_84 (impl)
macro_rules! Depcrate_errorimpl_84 {
() => {
// Module: crate::error
// Provides: {"impl_84"}
// Dependencies: {}
impl ser :: Error for ConfigError { fn custom < T : fmt :: Display > (msg : T) -> Self { Self :: Message (msg . to_string ()) } }
};
}
