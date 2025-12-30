// Generated macro for impl_550 (impl)
macro_rules! Depcrate_setimpl_550 {
() => {
// Module: crate::set
// Provides: {"impl_550"}
// Dependencies: {}
impl < T : fmt :: Debug , S , A : Allocator > fmt :: Debug for VacantEntry < '_ , T , S , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("VacantEntry") . field (self . get ()) . finish () } }
};
}
