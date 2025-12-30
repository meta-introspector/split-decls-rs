// Generated macro for impl_245 (impl)
macro_rules! Depcrate_paragraphimpl_245 {
() => {
// Module: crate::paragraph
// Provides: {"impl_245"}
// Dependencies: {}
impl Styled for Paragraph < '_ > { type Item = Self ; fn style (& self) -> Style { self . style } fn set_style < S : Into < Style > > (self , style : S) -> Self :: Item { self . style (style) } }
};
}
