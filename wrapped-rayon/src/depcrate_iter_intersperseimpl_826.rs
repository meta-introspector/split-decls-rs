// Generated macro for impl_826 (impl)
macro_rules! Depcrate_iter_intersperseimpl_826 {
() => {
// Module: crate::iter::intersperse
// Provides: {"impl_826"}
// Dependencies: {}
impl < I > ExactSizeIterator for IntersperseIter < I > where I : DoubleEndedIterator < Item : Clone > + ExactSizeIterator , { fn len (& self) -> usize { let len = self . base . len () ; len + len . saturating_sub (1) + self . clone_first as usize + self . clone_last as usize } }
};
}
