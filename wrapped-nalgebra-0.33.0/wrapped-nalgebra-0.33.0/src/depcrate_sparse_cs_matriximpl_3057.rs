// Generated macro for impl_3057 (impl)
macro_rules! Depcrate_sparse_cs_matriximpl_3057 {
() => {
// Module: crate::sparse::cs_matrix
// Provides: {"impl_3057"}
// Dependencies: {}
impl < 'a , T : Clone > Iterator for ColumnEntries < 'a , T > { type Item = (usize , T) ; # [inline] fn next (& mut self) -> Option < Self :: Item > { if self . curr >= self . i . len () { None } else { let res = Some ((unsafe { * self . i . get_unchecked (self . curr) } , unsafe { self . v . get_unchecked (self . curr) . clone () })) ; self . curr += 1 ; res } } }
};
}
