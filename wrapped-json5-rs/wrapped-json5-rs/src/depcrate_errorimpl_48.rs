// Generated macro for impl_48 (impl)
macro_rules! Depcrate_errorimpl_48 {
() => {
// Module: crate::error
// Provides: {"impl_48"}
// Dependencies: {}
impl de :: Error for Error { fn custom < T : Display > (msg : T) -> Self { Error :: Message { msg : msg . to_string () , location : None , } } }
};
}
