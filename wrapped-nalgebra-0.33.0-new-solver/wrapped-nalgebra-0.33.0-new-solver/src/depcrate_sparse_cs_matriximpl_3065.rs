// Generated macro for impl_3065 (impl)
macro_rules! Depcrate_sparse_cs_matriximpl_3065 {
() => {
// Module: crate::sparse::cs_matrix
// Provides: {"impl_3065"}
// Dependencies: {}
impl < 'a , T : Scalar , R : Dim , C : Dim > CsStorageIter < 'a , T , R , C > for CsVecStorage < T , R , C > where DefaultAllocator : Allocator < C > , { type ColumnEntries = ColumnEntries < 'a , T > ; type ColumnRowIndices = iter :: Cloned < slice :: Iter < 'a , usize > > ; # [inline] fn column_entries (& 'a self , j : usize) -> Self :: ColumnEntries { let rng = self . column_range (j) ; ColumnEntries :: new (& self . i [rng . clone ()] , & self . vals [rng]) } # [inline] fn column_row_indices (& 'a self , j : usize) -> Self :: ColumnRowIndices { let rng = self . column_range (j) ; self . i [rng] . iter () . cloned () } }
};
}
