// Generated macro for impl_569 (impl)
macro_rules! Depcrate_tableimpl_569 {
() => {
// Module: crate::table
// Provides: {"impl_569"}
// Dependencies: {}
impl < T : fmt :: Debug , A : Allocator > fmt :: Debug for Entry < '_ , T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Entry :: Vacant (ref v) => f . debug_tuple ("Entry") . field (v) . finish () , Entry :: Occupied (ref o) => f . debug_tuple ("Entry") . field (o) . finish () , } } }
};
}
