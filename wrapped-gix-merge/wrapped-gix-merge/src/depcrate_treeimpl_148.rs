// Generated macro for impl_148 (impl)
macro_rules! Depcrate_treeimpl_148 {
() => {
// Module: crate::tree
// Provides: {"impl_148"}
// Dependencies: {}
impl ConflictMapping { fn is_swapped (& self) -> bool { matches ! (self , ConflictMapping :: Swapped) } fn swapped (self) -> ConflictMapping { match self { ConflictMapping :: Original => ConflictMapping :: Swapped , ConflictMapping :: Swapped => ConflictMapping :: Original , } } fn to_global (self , global : ConflictMapping) -> ConflictMapping { match global { ConflictMapping :: Original => self , ConflictMapping :: Swapped => self . swapped () , } } }
};
}
