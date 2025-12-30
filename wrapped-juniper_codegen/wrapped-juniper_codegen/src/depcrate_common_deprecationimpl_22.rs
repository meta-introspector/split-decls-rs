// Generated macro for impl_22 (impl)
macro_rules! Depcrate_common_deprecationimpl_22 {
() => {
// Module: crate::common::deprecation
// Provides: {"impl_22"}
// Dependencies: {}
impl ToTokens for Directive { fn to_tokens (& self , into : & mut TokenStream) { let reason = self . reason . as_ref () . map_or_else (| | quote ! { <:: juniper :: ArcStr >:: None } , | text | quote ! { Some (:: juniper :: arcstr :: literal ! (# text)) } ,) ; quote ! { . deprecated (:: core :: option :: Option ::# reason) } . to_tokens (into) ; } }
};
}
