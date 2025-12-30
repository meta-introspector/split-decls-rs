// Generated macro for Path (struct)
macro_rules! Depcrate_hirPath {
() => {
// Module: crate::hir
// Provides: {"Path"}
// Dependencies: {}
# [doc = " A `Path` is essentially Rust's notion of a name; for instance,"] # [doc = " `std::cmp::PartialEq`. It's represented as a sequence of identifiers,"] # [doc = " along with a bunch of supporting information."] # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct Path < 'hir , R = Res > { pub span : Span , # [doc = " The resolution for the path."] pub res : R , # [doc = " The segments in the path: the things separated by `::`."] pub segments : & 'hir [PathSegment < 'hir >] , }
};
}
