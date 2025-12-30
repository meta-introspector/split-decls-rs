// Generated macro for impl_31 (impl)
macro_rules! Depcrate_arrayimpl_31 {
() => {
// Module: crate::array
// Provides: {"impl_31"}
// Dependencies: {}
impl < T : Type > Iterator for CFArrayIntoIter < T > { type Item = CFRetained < T > ; fn next (& mut self) -> Option < CFRetained < T > > { let value = self . array . get (self . index) ? ; self . index += 1 ; Some (value) } fn size_hint (& self) -> (usize , Option < usize >) { let len = self . array . len () . saturating_sub (self . index) ; (len , Some (len)) } }
};
}
