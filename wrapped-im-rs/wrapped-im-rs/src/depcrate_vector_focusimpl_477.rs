// Generated macro for impl_477 (impl)
macro_rules! Depcrate_vector_focusimpl_477 {
() => {
// Module: crate::vector::focus
// Provides: {"impl_477"}
// Dependencies: {}
impl < 'a , A > IntoIterator for FocusMut < 'a , A > where A : Clone + 'a , { type Item = & 'a mut A ; type IntoIter = IterMut < 'a , A > ; fn into_iter (self) -> Self :: IntoIter { IterMut :: from_focus (self) } }
};
}
