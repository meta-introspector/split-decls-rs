// Generated macro for impl_3092 (impl)
macro_rules! Depcrate_sparse_cs_matrix_conversionimpl_3092 {
() => {
// Module: crate::sparse::cs_matrix_conversion
// Provides: {"impl_3092"}
// Dependencies: {}
impl < 'a , T : Scalar + Zero + ClosedAddAssign , R : Dim , C : Dim > CsMatrix < T , R , C > where DefaultAllocator : Allocator < C > + Allocator < R > , { # [doc = " Creates a column-compressed sparse matrix from a sparse matrix in triplet form."] pub fn from_triplet_generic (nrows : R , ncols : C , irows : & [usize] , icols : & [usize] , vals : & [T] ,) -> Self { assert ! (vals . len () == irows . len ()) ; assert ! (vals . len () == icols . len ()) ; let mut res = CsMatrix :: new_uninitialized_generic (nrows , ncols , vals . len ()) ; let mut workspace = res . data . p . clone () ; for j in icols . iter () . cloned () { workspace [j] += 1 ; } let _ = cs_utils :: cumsum (& mut workspace , & mut res . data . p) ; for ((i , j) , val) in irows . iter () . cloned () . zip (icols . iter () . cloned ()) . zip (vals . iter () . cloned ()) { let offset = workspace [j] ; res . data . i [offset] = i ; res . data . vals [offset] = val ; workspace [j] = offset + 1 ; } res . sort () ; res . dedup () ; res } }
};
}
