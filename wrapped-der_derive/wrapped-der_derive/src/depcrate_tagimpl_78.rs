// Generated macro for impl_78 (impl)
macro_rules! Depcrate_tagimpl_78 {
() => {
// Module: crate::tag
// Provides: {"impl_78"}
// Dependencies: {}
impl Tag { # [doc = " Lower this [`Tag`] to a [`TokenStream`]."] pub fn to_tokens (self) -> TokenStream { match self { Tag :: Universal (ty) => ty . tag () , Tag :: Application { constructed , number , } => { let number = number . to_tokens () ; quote ! { :: der :: Tag :: Application { constructed : # constructed , number : # number , } } } Tag :: ContextSpecific { constructed , number , } => { let number = number . to_tokens () ; quote ! { :: der :: Tag :: ContextSpecific { constructed : # constructed , number : # number , } } } Tag :: Private { constructed , number , } => { let number = number . to_tokens () ; quote ! { :: der :: Tag :: Private { constructed : # constructed , number : # number , } } } } } }
};
}
