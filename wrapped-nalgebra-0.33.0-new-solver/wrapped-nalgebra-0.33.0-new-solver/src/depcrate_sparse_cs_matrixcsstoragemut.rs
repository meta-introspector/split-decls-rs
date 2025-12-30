// Generated macro for CsStorageMut (trait)
macro_rules! Depcrate_sparse_cs_matrixCsStorageMut {
() => {
// Module: crate::sparse::cs_matrix
// Provides: {"CsStorageMut"}
// Dependencies: {}
# [doc = " Trait for compressed column sparse matrix mutable storage."] pub trait CsStorageMut < T , R , C = U1 > : CsStorage < T , R , C > + for < 'a > CsStorageIterMut < 'a , T , R , C > { }
};
}
