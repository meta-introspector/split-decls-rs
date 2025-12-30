// Generated macro for impl_189 (impl)
macro_rules! Depcrate_listimpl_189 {
() => {
// Module: crate::list
// Provides: {"impl_189"}
// Dependencies: {}
impl Styled for List < '_ > { type Item = Self ; fn style (& self) -> Style { self . style } fn set_style < S : Into < Style > > (self , style : S) -> Self :: Item { self . style (style) } }
};
}
