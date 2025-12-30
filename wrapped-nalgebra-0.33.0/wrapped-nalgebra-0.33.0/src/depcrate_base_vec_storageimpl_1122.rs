// Generated macro for impl_1122 (impl)
macro_rules! Depcrate_base_vec_storageimpl_1122 {
() => {
// Module: crate::base::vec_storage
// Provides: {"impl_1122"}
// Dependencies: {}
impl < T , R , RV , SV > Extend < Vector < T , RV , SV > > for VecStorage < T , R , Dyn > where T : Scalar , R : Dim , RV : Dim , SV : RawStorage < T , RV > , ShapeConstraint : SameNumberOfRows < R , RV > , { # [doc = " Extends the number of columns of the `VecStorage` with vectors"] # [doc = " from the given iterator."] # [doc = ""] # [doc = " # Panics"] # [doc = " This function panics if the number of rows of each `Vector`"] # [doc = " yielded by the iterator is not equal to the number of rows"] # [doc = " of this `VecStorage`."] fn extend < I : IntoIterator < Item = Vector < T , RV , SV > > > (& mut self , iter : I) { let nrows = self . nrows . value () ; let iter = iter . into_iter () ; let (lower , _upper) = iter . size_hint () ; self . data . reserve (nrows * lower) ; for vector in iter { assert_eq ! (nrows , vector . shape () . 0) ; self . data . extend (vector . iter () . cloned ()) ; } self . ncols = Dyn (self . data . len () / nrows) ; } }
};
}
