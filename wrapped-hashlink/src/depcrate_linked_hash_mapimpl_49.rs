// Generated macro for impl_49 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_49 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_49"}
// Dependencies: {}
impl < K : fmt :: Debug , V : fmt :: Debug , S > fmt :: Debug for RawEntryMut < '_ , K , V , S > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { RawEntryMut :: Vacant (ref v) => f . debug_tuple ("RawEntry") . field (v) . finish () , RawEntryMut :: Occupied (ref o) => f . debug_tuple ("RawEntry") . field (o) . finish () , } } }
};
}
