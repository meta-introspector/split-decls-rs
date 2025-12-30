// Generated macro for impl_50 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_50 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_50"}
// Dependencies: {}
impl < K : fmt :: Debug , V : fmt :: Debug , S > fmt :: Debug for RawOccupiedEntryMut < '_ , K , V , S > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("RawOccupiedEntryMut") . field ("key" , self . key ()) . field ("value" , self . get ()) . finish () } }
};
}
