// Generated macro for impl_47 (impl)
macro_rules! Depcrate_errorimpl_47 {
() => {
// Module: crate::error
// Provides: {"impl_47"}
// Dependencies: {}
impl ser :: Error for Error { fn custom < T : Display > (msg : T) -> Self { Error :: Message { msg : msg . to_string () , location : None , } } }
};
}
