// Generated macro for impl_1006 (impl)
macro_rules! Depcrate_base_matrix_viewimpl_1006 {
() => {
// Module: crate::base::matrix_view
// Provides: {"impl_1006"}
// Dependencies: {}
impl < 'a , T , R1 , C1 , R2 , C2 > ReshapableStorage < T , R1 , C1 , R2 , C2 > for ViewStorage < 'a , T , R1 , C1 , U1 , R1 > where T : Scalar , R1 : Dim , C1 : Dim , R2 : Dim , C2 : Dim , { type Output = ViewStorage < 'a , T , R2 , C2 , U1 , R2 > ; fn reshape_generic (self , nrows : R2 , ncols : C2) -> Self :: Output { let (r1 , c1) = self . shape () ; assert_eq ! (nrows . value () * ncols . value () , r1 . value () * c1 . value ()) ; let ptr = self . ptr () ; let new_shape = (nrows , ncols) ; let strides = (U1 :: name () , nrows) ; unsafe { ViewStorage :: from_raw_parts (ptr , new_shape , strides) } } }
};
}
