// Generated macro for impl_548 (impl)
macro_rules! Depcrate_setimpl_548 {
() => {
// Module: crate::set
// Provides: {"impl_548"}
// Dependencies: {}
impl < T : fmt :: Debug , S , A : Allocator > fmt :: Debug for OccupiedEntry < '_ , T , S , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("OccupiedEntry") . field ("value" , self . get ()) . finish () } }
};
}
