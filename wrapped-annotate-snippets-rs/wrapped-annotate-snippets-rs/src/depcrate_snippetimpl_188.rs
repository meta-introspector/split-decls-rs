// Generated macro for impl_188 (impl)
macro_rules! Depcrate_snippetimpl_188 {
() => {
// Module: crate::snippet
// Provides: {"impl_188"}
// Dependencies: {}
impl < 'a , T : Into < Cow < 'a , str > > > From < Option < T > > for OptionCow < 'a > { fn from (value : Option < T >) -> Self { Self (value . map (Into :: into)) } }
};
}
