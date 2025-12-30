// Generated macro for impl_467 (impl)
macro_rules! Depcrate_vector_focusimpl_467 {
() => {
// Module: crate::vector::focus
// Provides: {"impl_467"}
// Dependencies: {}
impl < 'a , A > IntoIterator for Focus < 'a , A > where A : Clone + 'a , { type Item = & 'a A ; type IntoIter = Iter < 'a , A > ; fn into_iter (self) -> Self :: IntoIter { Iter :: from_focus (self) } }
};
}
