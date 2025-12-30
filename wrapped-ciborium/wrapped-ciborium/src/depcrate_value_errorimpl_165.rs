// Generated macro for impl_165 (impl)
macro_rules! Depcrate_value_errorimpl_165 {
() => {
// Module: crate::value::error
// Provides: {"impl_165"}
// Dependencies: {}
impl serde :: ser :: Error for Error { # [inline] fn custom < T : core :: fmt :: Display > (msg : T) -> Self { Self :: Custom (msg . to_string ()) } }
};
}
