// Generated macro for impl_283 (impl)
macro_rules! Depcrate_parseimpl_283 {
() => {
// Module: crate::parse
// Provides: {"impl_283"}
// Dependencies: {}
impl Parse for Attributes { fn parse (input : ParseStream) -> syn :: Result < Self > { let vars = Punctuated :: < Attribute , Token ! [::] > :: parse_terminated (input) ? ; Ok (Attributes { attributes : vars . into_iter () . collect () , }) } }
};
}
