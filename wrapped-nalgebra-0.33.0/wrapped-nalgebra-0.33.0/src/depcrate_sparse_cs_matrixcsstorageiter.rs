// Generated macro for CsStorageIter (trait)
macro_rules! Depcrate_sparse_cs_matrixCsStorageIter {
() => {
// Module: crate::sparse::cs_matrix
// Provides: {"CsStorageIter"}
// Dependencies: {}
# [doc = " Trait for iterable compressed-column matrix storage."] pub trait CsStorageIter < 'a , T , R , C = U1 > { # [doc = " Iterator through all the rows of a specific columns."] # [doc = ""] # [doc = " The elements are given as a tuple (`row_index`, value)."] type ColumnEntries : Iterator < Item = (usize , T) > ; # [doc = " Iterator through the row indices of a specific column."] type ColumnRowIndices : Iterator < Item = usize > ; # [doc = " Iterates through all the row indices of the j-th column."] fn column_row_indices (& 'a self , j : usize) -> Self :: ColumnRowIndices ; # [doc = " Iterates through all the entries of the j-th column."] fn column_entries (& 'a self , j : usize) -> Self :: ColumnEntries ; }
};
}
