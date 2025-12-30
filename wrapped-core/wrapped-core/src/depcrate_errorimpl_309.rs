// Generated macro for impl_309 (impl)
macro_rules! Depcrate_errorimpl_309 {
() => {
// Module: crate::error
// Provides: {"impl_309"}
// Dependencies: {}
impl Error { # [doc = " Create a new error about a literal string that doesn't match a set of known"] # [doc = " or permissible values. This function can be made public if the API proves useful"] # [doc = " beyond impls for `syn` types."] pub (crate) fn unknown_lit_str_value (value : & LitStr) -> Self { Error :: unknown_value (& value . value ()) . with_span (value) } }
};
}
