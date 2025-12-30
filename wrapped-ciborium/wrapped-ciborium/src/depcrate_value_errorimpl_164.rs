// Generated macro for impl_164 (impl)
macro_rules! Depcrate_value_errorimpl_164 {
() => {
// Module: crate::value::error
// Provides: {"impl_164"}
// Dependencies: {}
impl serde :: de :: Error for Error { # [inline] fn custom < T : core :: fmt :: Display > (msg : T) -> Self { Self :: Custom (msg . to_string ()) } }
};
}
