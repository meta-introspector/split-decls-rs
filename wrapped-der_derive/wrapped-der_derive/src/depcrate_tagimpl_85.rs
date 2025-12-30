// Generated macro for impl_85 (impl)
macro_rules! Depcrate_tagimpl_85 {
() => {
// Module: crate::tag
// Provides: {"impl_85"}
// Dependencies: {}
impl TagNumber { # [doc = " Get tokens describing this tag."] pub fn to_tokens (self) -> TokenStream { let num = self . 0 ; quote ! (:: der :: TagNumber (# num)) } }
};
}
