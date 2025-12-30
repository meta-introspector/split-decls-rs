// Generated macro for impl_1115 (impl)
macro_rules! Depcrate_base_vec_storageimpl_1115 {
() => {
// Module: crate::base::vec_storage
// Provides: {"impl_1115"}
// Dependencies: {}
impl < T , C1 , C2 > ReshapableStorage < T , Dyn , C1 , Dyn , C2 > for VecStorage < T , Dyn , C1 > where T : Scalar , C1 : Dim , C2 : Dim , { type Output = VecStorage < T , Dyn , C2 > ; fn reshape_generic (self , nrows : Dyn , ncols : C2) -> Self :: Output { assert_eq ! (nrows . value () * ncols . value () , self . data . len ()) ; VecStorage { data : self . data , nrows , ncols , } } }
};
}
