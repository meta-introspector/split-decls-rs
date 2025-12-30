// Generated macro for impl_1118 (impl)
macro_rules! Depcrate_base_vec_storageimpl_1118 {
() => {
// Module: crate::base::vec_storage
// Provides: {"impl_1118"}
// Dependencies: {}
impl < T , R1 , C2 > ReshapableStorage < T , R1 , Dyn , Dyn , C2 > for VecStorage < T , R1 , Dyn > where T : Scalar , R1 : DimName , C2 : Dim , { type Output = VecStorage < T , Dyn , C2 > ; fn reshape_generic (self , nrows : Dyn , ncols : C2) -> Self :: Output { assert_eq ! (nrows . value () * ncols . value () , self . data . len ()) ; VecStorage { data : self . data , nrows , ncols , } } }
};
}
