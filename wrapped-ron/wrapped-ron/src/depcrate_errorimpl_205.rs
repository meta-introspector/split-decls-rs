// Generated macro for impl_205 (impl)
macro_rules! Depcrate_errorimpl_205 {
() => {
// Module: crate::error
// Provides: {"impl_205"}
// Dependencies: {}
impl ser :: Error for Error { # [cold] fn custom < T : fmt :: Display > (msg : T) -> Self { Error :: Message (msg . to_string ()) } }
};
}
