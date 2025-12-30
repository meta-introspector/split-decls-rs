// Generated macro for impl_825 (impl)
macro_rules! Depcrate_iter_intersperseimpl_825 {
() => {
// Module: crate::iter::intersperse
// Provides: {"impl_825"}
// Dependencies: {}
impl < I > DoubleEndedIterator for IntersperseIter < I > where I : DoubleEndedIterator < Item : Clone > + ExactSizeIterator , { fn next_back (& mut self) -> Option < Self :: Item > { if self . clone_last { self . clone_last = false ; Some (self . item . clone ()) } else if let next_back @ Some (_) = self . base . next_back () { self . clone_last = self . base . len () != 0 ; next_back } else if self . clone_first { self . clone_first = false ; Some (self . item . clone ()) } else { None } } }
};
}
