// Generated macro for impl_546 (impl)
macro_rules! Depcrate_setimpl_546 {
() => {
// Module: crate::set
// Provides: {"impl_546"}
// Dependencies: {}
impl < T : fmt :: Debug , S , A : Allocator > fmt :: Debug for Entry < '_ , T , S , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Entry :: Vacant (ref v) => f . debug_tuple ("Entry") . field (v) . finish () , Entry :: Occupied (ref o) => f . debug_tuple ("Entry") . field (o) . finish () , } } }
};
}
