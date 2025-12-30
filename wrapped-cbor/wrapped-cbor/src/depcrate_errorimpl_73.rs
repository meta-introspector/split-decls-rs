// Generated macro for impl_73 (impl)
macro_rules! Depcrate_errorimpl_73 {
() => {
// Module: crate::error
// Provides: {"impl_73"}
// Dependencies: {}
impl ser :: Error for Error { fn custom < T : fmt :: Display > (msg : T) -> Error { Error :: message (msg) } }
};
}
