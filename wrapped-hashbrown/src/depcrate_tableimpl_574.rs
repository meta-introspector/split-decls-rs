// Generated macro for impl_574 (impl)
macro_rules! Depcrate_tableimpl_574 {
() => {
// Module: crate::table
// Provides: {"impl_574"}
// Dependencies: {}
impl < T : fmt :: Debug , A : Allocator > fmt :: Debug for OccupiedEntry < '_ , T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("OccupiedEntry") . field ("value" , self . get ()) . finish () } }
};
}
