// Generated macro for impl_431 (impl)
macro_rules! Depcrate_raw_entryimpl_431 {
() => {
// Module: crate::raw_entry
// Provides: {"impl_431"}
// Dependencies: {}
impl < K : Debug , V : Debug , S , A : Allocator > Debug for RawEntryMut < '_ , K , V , S , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { RawEntryMut :: Vacant (ref v) => f . debug_tuple ("RawEntry") . field (v) . finish () , RawEntryMut :: Occupied (ref o) => f . debug_tuple ("RawEntry") . field (o) . finish () , } } }
};
}
