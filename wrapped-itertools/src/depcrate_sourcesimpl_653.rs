// Generated macro for impl_653 (impl)
macro_rules! Depcrate_sourcesimpl_653 {
() => {
// Module: crate::sources
// Provides: {"impl_653"}
// Dependencies: {}
impl < A , St , F > Iterator for Unfold < St , F > where F : FnMut (& mut St) -> Option < A > , { type Item = A ; # [inline] fn next (& mut self) -> Option < Self :: Item > { (self . f) (& mut self . state) } }
};
}
