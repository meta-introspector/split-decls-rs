// Generated macro for impl_183 (impl)
macro_rules! Depcrate_stackvecimpl_183 {
() => {
// Module: crate::stackvec
// Provides: {"impl_183"}
// Dependencies: {}
impl ops :: MulAssign < & [bigint :: Limb] > for StackVec { # [inline] fn mul_assign (& mut self , rhs : & [bigint :: Limb]) { bigint :: large_mul (self , rhs) . unwrap () ; } }
};
}
