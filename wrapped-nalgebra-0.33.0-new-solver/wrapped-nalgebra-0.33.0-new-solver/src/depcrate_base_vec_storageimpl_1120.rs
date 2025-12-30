// Generated macro for impl_1120 (impl)
macro_rules! Depcrate_base_vec_storageimpl_1120 {
() => {
// Module: crate::base::vec_storage
// Provides: {"impl_1120"}
// Dependencies: {}
impl < T , R : Dim > Extend < T > for VecStorage < T , R , Dyn > { # [doc = " Extends the number of columns of the `VecStorage` with elements"] # [doc = " from the given iterator."] # [doc = ""] # [doc = " # Panics"] # [doc = " This function panics if the number of elements yielded by the"] # [doc = " given iterator is not a multiple of the number of rows of the"] # [doc = " `VecStorage`."] fn extend < I : IntoIterator < Item = T > > (& mut self , iter : I) { self . data . extend (iter) ; self . ncols = Dyn (self . data . len () / self . nrows . value ()) ; assert ! (self . data . len () % self . nrows . value () == 0 , "The number of elements produced by the given iterator was not a multiple of the number of rows.") ; } }
};
}
