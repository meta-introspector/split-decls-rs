// Generated macro for impl_1102 (impl)
macro_rules! Depcrate_base_vec_storageimpl_1102 {
() => {
// Module: crate::base::vec_storage
// Provides: {"impl_1102"}
// Dependencies: {}
impl < T , C : DimName > Default for VecStorage < T , Dyn , C > { fn default () -> Self { Self { data : Vec :: new () , nrows : Dyn :: from_usize (0) , ncols : C :: name () , } } }
};
}
