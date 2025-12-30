// Generated macro for impl_1116 (impl)
macro_rules! Depcrate_base_vec_storageimpl_1116 {
() => {
// Module: crate::base::vec_storage
// Provides: {"impl_1116"}
// Dependencies: {}
impl < T , C1 , R2 > ReshapableStorage < T , Dyn , C1 , R2 , Dyn > for VecStorage < T , Dyn , C1 > where T : Scalar , C1 : Dim , R2 : DimName , { type Output = VecStorage < T , R2 , Dyn > ; fn reshape_generic (self , nrows : R2 , ncols : Dyn) -> Self :: Output { assert_eq ! (nrows . value () * ncols . value () , self . data . len ()) ; VecStorage { data : self . data , nrows , ncols , } } }
};
}
