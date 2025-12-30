// Generated macro for impl_247 (impl)
macro_rules! Depcrate_base_opsimpl_247 {
() => {
// Module: crate::base::ops
// Provides: {"impl_247"}
// Dependencies: {}
impl < T , R : Dim , C : Dim , S : RawStorageMut < T , R , C > > IndexMut < usize > for Matrix < T , R , C , S > { # [inline] fn index_mut (& mut self , i : usize) -> & mut T { let ij = self . vector_to_matrix_index (i) ; & mut self [ij] } }
};
}
