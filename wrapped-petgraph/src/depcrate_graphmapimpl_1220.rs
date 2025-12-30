// Generated macro for impl_1220 (impl)
macro_rules! Depcrate_graphmapimpl_1220 {
() => {
// Module: crate::graphmap
// Provides: {"impl_1220"}
// Dependencies: {}
impl < 'b , T > Ord for Ptr < 'b , T > { # [doc = " Ptr is ordered by pointer value, i.e. an arbitrary but stable and total order."] fn cmp (& self , other : & Ptr < 'b , T >) -> Ordering { let a : * const T = self . 0 ; let b : * const T = other . 0 ; a . cmp (& b) } }
};
}
