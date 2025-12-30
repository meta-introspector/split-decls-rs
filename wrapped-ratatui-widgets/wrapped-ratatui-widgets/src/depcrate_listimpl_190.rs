// Generated macro for impl_190 (impl)
macro_rules! Depcrate_listimpl_190 {
() => {
// Module: crate::list
// Provides: {"impl_190"}
// Dependencies: {}
impl Styled for ListItem < '_ > { type Item = Self ; fn style (& self) -> Style { self . style } fn set_style < S : Into < Style > > (self , style : S) -> Self :: Item { self . style (style) } }
};
}
