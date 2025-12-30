// Generated macro for lower_mutability (function)
macro_rules! Depcrate_lowerlower_mutability {
() => {
// Module: crate::lower
// Provides: {"lower_mutability"}
// Dependencies: {}
pub (crate) fn lower_mutability (m : hir_def :: type_ref :: Mutability) -> Mutability { match m { hir_def :: type_ref :: Mutability :: Shared => Mutability :: Not , hir_def :: type_ref :: Mutability :: Mut => Mutability :: Mut , } }
};
}
