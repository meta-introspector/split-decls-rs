// Generated macro for impl_28 (impl)
macro_rules! Depcrate_arrayimpl_28 {
() => {
// Module: crate::array
// Provides: {"impl_28"}
// Dependencies: {}
impl < T : Type > Iterator for CFArrayIter < '_ , T > { type Item = CFRetained < T > ; fn next (& mut self) -> Option < CFRetained < T > > { let value = self . array . get (self . index) ? ; self . index += 1 ; Some (value) } fn size_hint (& self) -> (usize , Option < usize >) { let len = self . array . len () . saturating_sub (self . index) ; (len , Some (len)) } }
};
}
