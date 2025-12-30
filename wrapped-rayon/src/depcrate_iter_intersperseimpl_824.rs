// Generated macro for impl_824 (impl)
macro_rules! Depcrate_iter_intersperseimpl_824 {
() => {
// Module: crate::iter::intersperse
// Provides: {"impl_824"}
// Dependencies: {}
impl < I > Iterator for IntersperseIter < I > where I : DoubleEndedIterator < Item : Clone > + ExactSizeIterator , { type Item = I :: Item ; fn next (& mut self) -> Option < Self :: Item > { if self . clone_first { self . clone_first = false ; Some (self . item . clone ()) } else if let next @ Some (_) = self . base . next () { self . clone_first = self . base . len () != 0 ; next } else if self . clone_last { self . clone_last = false ; Some (self . item . clone ()) } else { None } } fn size_hint (& self) -> (usize , Option < usize >) { let len = self . len () ; (len , Some (len)) } }
};
}
