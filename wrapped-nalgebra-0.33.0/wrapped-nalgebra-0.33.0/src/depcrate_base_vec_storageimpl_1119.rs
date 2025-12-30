// Generated macro for impl_1119 (impl)
macro_rules! Depcrate_base_vec_storageimpl_1119 {
() => {
// Module: crate::base::vec_storage
// Provides: {"impl_1119"}
// Dependencies: {}
impl < T , R1 , R2 > ReshapableStorage < T , R1 , Dyn , R2 , Dyn > for VecStorage < T , R1 , Dyn > where T : Scalar , R1 : DimName , R2 : DimName , { type Output = VecStorage < T , R2 , Dyn > ; fn reshape_generic (self , nrows : R2 , ncols : Dyn) -> Self :: Output { assert_eq ! (nrows . value () * ncols . value () , self . data . len ()) ; VecStorage { data : self . data , nrows , ncols , } } }
};
}
