// Generated macro for impl_3056 (impl)
macro_rules! Depcrate_sparse_cs_matriximpl_3056 {
() => {
// Module: crate::sparse::cs_matrix
// Provides: {"impl_3056"}
// Dependencies: {}
impl < 'a , T > ColumnEntries < 'a , T > { # [inline] pub fn new (i : & 'a [usize] , v : & 'a [T]) -> Self { assert_eq ! (i . len () , v . len ()) ; Self { curr : 0 , i , v } } }
};
}
