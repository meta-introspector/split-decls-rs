// Generated macro for impl_32 (impl)
macro_rules! Depcrate_common_descriptionimpl_32 {
() => {
// Module: crate::common::description
// Provides: {"impl_32"}
// Dependencies: {}
impl ToTokens for Description { fn to_tokens (& self , into : & mut TokenStream) { let desc = & self . 0 ; quote ! { . description (:: juniper :: arcstr :: literal ! (# desc)) } . to_tokens (into) ; } }
};
}
