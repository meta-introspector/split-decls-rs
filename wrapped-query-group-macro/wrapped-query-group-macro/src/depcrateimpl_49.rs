// Generated macro for impl_49 (impl)
macro_rules! Depcrateimpl_49 {
() => {
// Module: crate
// Provides: {"impl_49"}
// Dependencies: {}
impl < T > syn :: parse :: Parse for Parenthesized < T > where T : syn :: parse :: Parse , { fn parse (input : ParseStream < '_ >) -> syn :: Result < Self > { let content ; syn :: parenthesized ! (content in input) ; content . parse :: < T > () . map (Parenthesized) } }
};
}
