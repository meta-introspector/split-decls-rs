// Generated macro for impl_1101 (impl)
macro_rules! Depcrate_base_vec_storageimpl_1101 {
() => {
// Module: crate::base::vec_storage
// Provides: {"impl_1101"}
// Dependencies: {}
impl < T , R : DimName > Default for VecStorage < T , R , Dyn > { fn default () -> Self { Self { data : Vec :: new () , nrows : R :: name () , ncols : Dyn :: from_usize (0) , } } }
};
}
