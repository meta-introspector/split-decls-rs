// Generated macro for impl_80 (impl)
macro_rules! Depcrate_tagimpl_80 {
() => {
// Module: crate::tag
// Provides: {"impl_80"}
// Dependencies: {}
impl TagMode { # [doc = " Lower this [`TagMode`] to a [`TokenStream`] with the `der`"] # [doc = " crate's corresponding enum variant for this tag mode."] pub fn to_tokens (self) -> TokenStream { match self { TagMode :: Explicit => quote ! (:: der :: TagMode :: Explicit) , TagMode :: Implicit => quote ! (:: der :: TagMode :: Implicit) , } } }
};
}
