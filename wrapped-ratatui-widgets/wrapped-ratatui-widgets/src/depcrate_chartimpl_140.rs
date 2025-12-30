// Generated macro for impl_140 (impl)
macro_rules! Depcrate_chartimpl_140 {
() => {
// Module: crate::chart
// Provides: {"impl_140"}
// Dependencies: {}
impl Styled for Chart < '_ > { type Item = Self ; fn style (& self) -> Style { self . style } fn set_style < S : Into < Style > > (self , style : S) -> Self :: Item { self . style (style) } }
};
}
