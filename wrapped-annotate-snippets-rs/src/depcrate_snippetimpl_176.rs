// Generated macro for impl_176 (impl)
macro_rules! Depcrate_snippetimpl_176 {
() => {
// Module: crate::snippet
// Provides: {"impl_176"}
// Dependencies: {}
impl < 'a > Snippet < 'a , Annotation < 'a > > { # [doc = " Highlight and describe a span of text within the [`source`][Self::source]"] pub fn annotation (mut self , annotation : Annotation < 'a >) -> Snippet < 'a , Annotation < 'a > > { self . markers . push (annotation) ; self } # [doc = " Highlight and describe spans of text within the [`source`][Self::source]"] pub fn annotations (mut self , annotation : impl IntoIterator < Item = Annotation < 'a > >) -> Self { self . markers . extend (annotation) ; self } }
};
}
