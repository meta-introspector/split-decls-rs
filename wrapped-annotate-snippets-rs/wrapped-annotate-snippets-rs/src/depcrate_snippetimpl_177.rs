// Generated macro for impl_177 (impl)
macro_rules! Depcrate_snippetimpl_177 {
() => {
// Module: crate::snippet
// Provides: {"impl_177"}
// Dependencies: {}
impl < 'a > Snippet < 'a , Patch < 'a > > { # [doc = " Suggest to the user an edit to the [`source`][Self::source]"] pub fn patch (mut self , patch : Patch < 'a >) -> Snippet < 'a , Patch < 'a > > { self . markers . push (patch) ; self } # [doc = " Suggest to the user edits to the [`source`][Self::source]"] pub fn patches (mut self , patches : impl IntoIterator < Item = Patch < 'a > >) -> Self { self . markers . extend (patches) ; self } }
};
}
