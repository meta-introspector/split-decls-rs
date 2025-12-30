// Generated macro for impl_322 (impl)
macro_rules! Depcrate_tableimpl_322 {
() => {
// Module: crate::table
// Provides: {"impl_322"}
// Dependencies: {}
impl Styled for Table < '_ > { type Item = Self ; fn style (& self) -> Style { self . style } fn set_style < S : Into < Style > > (self , style : S) -> Self :: Item { self . style (style) } }
};
}
