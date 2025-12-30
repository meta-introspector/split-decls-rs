// Generated macro for impl_556 (impl)
macro_rules! Depcrate_diffimpl_556 {
() => {
// Module: crate::diff
// Provides: {"impl_556"}
// Dependencies: {}
impl Binding for DiffBinaryKind { type Raw = raw :: git_diff_binary_t ; unsafe fn from_raw (raw : raw :: git_diff_binary_t) -> DiffBinaryKind { match raw { raw :: GIT_DIFF_BINARY_NONE => DiffBinaryKind :: None , raw :: GIT_DIFF_BINARY_LITERAL => DiffBinaryKind :: Literal , raw :: GIT_DIFF_BINARY_DELTA => DiffBinaryKind :: Delta , _ => panic ! ("Unknown git diff binary kind") , } } fn raw (& self) -> raw :: git_diff_binary_t { match * self { DiffBinaryKind :: None => raw :: GIT_DIFF_BINARY_NONE , DiffBinaryKind :: Literal => raw :: GIT_DIFF_BINARY_LITERAL , DiffBinaryKind :: Delta => raw :: GIT_DIFF_BINARY_DELTA , } } }
};
}
