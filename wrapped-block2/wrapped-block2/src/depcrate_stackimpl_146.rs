// Generated macro for impl_146 (impl)
macro_rules! Depcrate_stackimpl_146 {
() => {
// Module: crate::stack
// Provides: {"impl_146"}
// Dependencies: {}
impl < A , R , Closure > fmt :: Debug for StackBlock < '_ , A , R , Closure > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut f = f . debug_struct ("StackBlock") ; debug_block_header (& self . header , & mut f) ; f . finish_non_exhaustive () } }
};
}
