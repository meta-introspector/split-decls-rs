// Generated macro for impl_3063 (impl)
macro_rules! Depcrate_sparse_cs_matriximpl_3063 {
() => {
// Module: crate::sparse::cs_matrix
// Provides: {"impl_3063"}
// Dependencies: {}
impl < T : Scalar , R : Dim , C : Dim > CsVecStorage < T , R , C > where DefaultAllocator : Allocator < C > , { # [doc = " The value buffer of this storage."] # [must_use] pub fn values (& self) -> & [T] { & self . vals } # [doc = " The column shifts buffer."] # [must_use] pub fn p (& self) -> & [usize] { self . p . as_slice () } # [doc = " The row index buffers."] # [must_use] pub fn i (& self) -> & [usize] { & self . i } }
};
}
