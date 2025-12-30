// Generated macro for impl_1121 (impl)
macro_rules! Depcrate_base_vec_storageimpl_1121 {
() => {
// Module: crate::base::vec_storage
// Provides: {"impl_1121"}
// Dependencies: {}
impl < 'a , T : 'a + Copy , R : Dim > Extend < & 'a T > for VecStorage < T , R , Dyn > { # [doc = " Extends the number of columns of the `VecStorage` with elements"] # [doc = " from the given iterator."] # [doc = ""] # [doc = " # Panics"] # [doc = " This function panics if the number of elements yielded by the"] # [doc = " given iterator is not a multiple of the number of rows of the"] # [doc = " `VecStorage`."] fn extend < I : IntoIterator < Item = & 'a T > > (& mut self , iter : I) { self . extend (iter . into_iter () . copied ()) } }
};
}
