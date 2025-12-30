// Generated macro for impl_245 (impl)
macro_rules! Depcrate_base_opsimpl_245 {
() => {
// Module: crate::base::ops
// Provides: {"impl_245"}
// Dependencies: {}
impl < T , R : Dim , C : Dim , S : RawStorage < T , R , C > > Index < usize > for Matrix < T , R , C , S > { type Output = T ; # [inline] fn index (& self , i : usize) -> & Self :: Output { let ij = self . vector_to_matrix_index (i) ; & self [ij] } }
};
}
