// Generated macro for impl_27 (impl)
macro_rules! Depcrate_matches_patternimpl_27 {
() => {
// Module: crate::matches_pattern
// Provides: {"impl_27"}
// Dependencies: {}
impl Parse for FieldOrMethod { # [doc = " Parses the field name or method call along with the `:` that follows it."] fn parse (input : ParseStream) -> syn :: Result < Self > { let value = if input . peek2 (Token ! [:]) && ! input . peek2 (Token ! [::]) { input . parse () . map (FieldOrMethod :: Field) } else { input . parse () . map (FieldOrMethod :: Method) } ? ; input . parse :: < Token ! [:] > () ? ; Ok (value) } }
};
}
