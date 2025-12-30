// Generated macro for IntersperseIter (struct)
macro_rules! Depcrate_iter_intersperseIntersperseIter {
() => {
// Module: crate::iter::intersperse
// Provides: {"IntersperseIter"}
// Dependencies: {}
struct IntersperseIter < I > where I : Iterator , { base : Fuse < I > , item : I :: Item , clone_first : bool , clone_last : bool , }
};
}
