// Generated macro for impl_145 (impl)
macro_rules! Depcrate_encodeimpl_145 {
() => {
// Module: crate::encode
// Provides: {"impl_145"}
// Dependencies: {}
impl serde :: ser :: Error for Error { # [doc = " Raised when there is general error when deserializing a type."] # [cold] fn custom < T : Display > (msg : T) -> Self { Self :: Syntax (msg . to_string ()) } }
};
}
