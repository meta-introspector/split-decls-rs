// Generated macro for CsStorage (trait)
macro_rules! Depcrate_sparse_cs_matrixCsStorage {
() => {
// Module: crate::sparse::cs_matrix
// Provides: {"CsStorage"}
// Dependencies: {}
# [doc = " Trait for compressed column sparse matrix storage."] pub trait CsStorage < T , R , C = U1 > : for < 'a > CsStorageIter < 'a , T , R , C > { # [doc = " The shape of the stored matrix."] fn shape (& self) -> (R , C) ; # [doc = " Retrieve the i-th row index of the underlying row index buffer."] # [doc = ""] # [doc = " # Safety"] # [doc = " No bound-checking is performed."] unsafe fn row_index_unchecked (& self , i : usize) -> usize ; # [doc = " The i-th value on the contiguous value buffer of this storage."] # [doc = ""] # [doc = " # Safety"] # [doc = " No bound-checking is performed."] unsafe fn get_value_unchecked (& self , i : usize) -> & T ; # [doc = " The i-th value on the contiguous value buffer of this storage."] fn get_value (& self , i : usize) -> & T ; # [doc = " Retrieve the i-th row index of the underlying row index buffer."] fn row_index (& self , i : usize) -> usize ; # [doc = " The value indices for the `i`-th column."] fn column_range (& self , i : usize) -> Range < usize > ; # [doc = " The size of the value buffer (i.e. the entries known as possibly being non-zero)."] fn len (& self) -> usize ; }
};
}
