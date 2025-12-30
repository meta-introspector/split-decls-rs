// Generated macro for impl_1103 (impl)
macro_rules! Depcrate_base_vec_storageimpl_1103 {
() => {
// Module: crate::base::vec_storage
// Provides: {"impl_1103"}
// Dependencies: {}
impl < T : Default , R : DimName , C : DimName > Default for VecStorage < T , R , C > { fn default () -> Self { let nrows = R :: name () ; let ncols = C :: name () ; let mut data = Vec :: new () ; data . resize_with (nrows . value () * ncols . value () , Default :: default) ; Self { data , nrows , ncols } } }
};
}
