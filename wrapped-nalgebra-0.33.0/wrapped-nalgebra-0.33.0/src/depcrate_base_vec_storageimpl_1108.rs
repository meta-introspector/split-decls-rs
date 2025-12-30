// Generated macro for impl_1108 (impl)
macro_rules! Depcrate_base_vec_storageimpl_1108 {
() => {
// Module: crate::base::vec_storage
// Provides: {"impl_1108"}
// Dependencies: {}
impl < T , R : Dim , C : Dim > From < VecStorage < T , R , C > > for Vec < T > { fn from (vec : VecStorage < T , R , C >) -> Self { vec . data } }
};
}
