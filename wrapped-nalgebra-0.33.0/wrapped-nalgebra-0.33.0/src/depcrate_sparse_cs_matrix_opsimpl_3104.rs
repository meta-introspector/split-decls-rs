// Generated macro for impl_3104 (impl)
macro_rules! Depcrate_sparse_cs_matrix_opsimpl_3104 {
() => {
// Module: crate::sparse::cs_matrix_ops
// Provides: {"impl_3104"}
// Dependencies: {}
impl < T : Scalar , R : Dim , C : Dim , S : CsStorage < T , R , C > > CsMatrix < T , R , C , S > { fn scatter < R2 : Dim , C2 : Dim > (& self , j : usize , beta : T , timestamps : & mut [usize] , timestamp : usize , workspace : & mut [T] , mut nz : usize , res : & mut CsMatrix < T , R2 , C2 > ,) -> usize where T : ClosedAddAssign + ClosedMulAssign , DefaultAllocator : Allocator < C2 > , { for (i , val) in self . data . column_entries (j) { if timestamps [i] < timestamp { timestamps [i] = timestamp ; res . data . i [nz] = i ; nz += 1 ; workspace [i] = val * beta . clone () ; } else { workspace [i] += val * beta . clone () ; } } nz } }
};
}
