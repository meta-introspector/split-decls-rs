// Generated macro for CsStorageIterMut (trait)
macro_rules! Depcrate_sparse_cs_matrixCsStorageIterMut {
() => {
// Module: crate::sparse::cs_matrix
// Provides: {"CsStorageIterMut"}
// Dependencies: {}
# [doc = " Trait for mutably iterable compressed-column sparse matrix storage."] pub trait CsStorageIterMut < 'a , T : 'a , R , C = U1 > { # [doc = " Mutable iterator through all the values of the sparse matrix."] type ValuesMut : Iterator < Item = & 'a mut T > ; # [doc = " Mutable iterator through all the rows of a specific columns."] # [doc = ""] # [doc = " The elements are given as a tuple (`row_index`, value)."] type ColumnEntriesMut : Iterator < Item = (usize , & 'a mut T) > ; # [doc = " A mutable iterator through the values buffer of the sparse matrix."] fn values_mut (& 'a mut self) -> Self :: ValuesMut ; # [doc = " Iterates mutably through all the entries of the j-th column."] fn column_entries_mut (& 'a mut self , j : usize) -> Self :: ColumnEntriesMut ; }
};
}
