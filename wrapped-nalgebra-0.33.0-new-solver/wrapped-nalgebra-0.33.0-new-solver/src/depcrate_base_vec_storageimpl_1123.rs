// Generated macro for impl_1123 (impl)
macro_rules! Depcrate_base_vec_storageimpl_1123 {
() => {
// Module: crate::base::vec_storage
// Provides: {"impl_1123"}
// Dependencies: {}
impl < T > Extend < T > for VecStorage < T , Dyn , U1 > { # [doc = " Extends the number of rows of the `VecStorage` with elements"] # [doc = " from the given iterator."] fn extend < I : IntoIterator < Item = T > > (& mut self , iter : I) { self . data . extend (iter) ; self . nrows = Dyn (self . data . len ()) ; } }
};
}
