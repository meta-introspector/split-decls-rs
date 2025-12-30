// Generated macro for impl_517 (impl)
macro_rules! Depcrate_options_forward_attrsimpl_517 {
() => {
// Module: crate::options::forward_attrs
// Provides: {"impl_517"}
// Dependencies: {}
impl ForwardAttrsFilter { # [doc = " Returns `true` if this will not forward any attributes."] pub fn is_empty (& self) -> bool { match * self { ForwardAttrsFilter :: All => false , ForwardAttrsFilter :: Only (ref list) => list . is_empty () , } } }
};
}
