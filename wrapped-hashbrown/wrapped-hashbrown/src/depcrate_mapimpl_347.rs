// Generated macro for impl_347 (impl)
macro_rules! Depcrate_mapimpl_347 {
() => {
// Module: crate::map
// Provides: {"impl_347"}
// Dependencies: {}
impl < K : Debug , V : Debug , S , A : Allocator > Debug for Entry < '_ , K , V , S , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Entry :: Vacant (ref v) => f . debug_tuple ("Entry") . field (v) . finish () , Entry :: Occupied (ref o) => f . debug_tuple ("Entry") . field (o) . finish () , } } }
};
}
