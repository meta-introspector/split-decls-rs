// Generated macro for impl_30 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_30 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_30"}
// Dependencies: {}
impl < K : fmt :: Debug , V : fmt :: Debug , S > fmt :: Debug for Entry < '_ , K , V , S > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Entry :: Vacant (ref v) => f . debug_tuple ("Entry") . field (v) . finish () , Entry :: Occupied (ref o) => f . debug_tuple ("Entry") . field (o) . finish () , } } }
};
}
