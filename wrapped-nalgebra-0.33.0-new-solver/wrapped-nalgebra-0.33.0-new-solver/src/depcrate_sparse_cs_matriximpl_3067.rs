// Generated macro for impl_3067 (impl)
macro_rules! Depcrate_sparse_cs_matriximpl_3067 {
() => {
// Module: crate::sparse::cs_matrix
// Provides: {"impl_3067"}
// Dependencies: {}
impl < 'a , T : Scalar , R : Dim , C : Dim > CsStorageIterMut < 'a , T , R , C > for CsVecStorage < T , R , C > where DefaultAllocator : Allocator < C > , { type ValuesMut = slice :: IterMut < 'a , T > ; type ColumnEntriesMut = iter :: Zip < iter :: Cloned < slice :: Iter < 'a , usize > > , slice :: IterMut < 'a , T > > ; # [inline] fn values_mut (& 'a mut self) -> Self :: ValuesMut { self . vals . iter_mut () } # [inline] fn column_entries_mut (& 'a mut self , j : usize) -> Self :: ColumnEntriesMut { let rng = self . column_range (j) ; self . i [rng . clone ()] . iter () . cloned () . zip (self . vals [rng] . iter_mut ()) } }
};
}
