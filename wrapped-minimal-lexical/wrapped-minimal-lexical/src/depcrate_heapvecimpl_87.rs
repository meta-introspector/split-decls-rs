// Generated macro for impl_87 (impl)
macro_rules! Depcrate_heapvecimpl_87 {
() => {
// Module: crate::heapvec
// Provides: {"impl_87"}
// Dependencies: {}
impl ops :: MulAssign < & [bigint :: Limb] > for HeapVec { # [inline] fn mul_assign (& mut self , rhs : & [bigint :: Limb]) { bigint :: large_mul (self , rhs) . unwrap () ; } }
};
}
