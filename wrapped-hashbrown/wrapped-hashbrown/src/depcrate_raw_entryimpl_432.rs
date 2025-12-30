// Generated macro for impl_432 (impl)
macro_rules! Depcrate_raw_entryimpl_432 {
() => {
// Module: crate::raw_entry
// Provides: {"impl_432"}
// Dependencies: {}
impl < K : Debug , V : Debug , S , A : Allocator > Debug for RawOccupiedEntryMut < '_ , K , V , S , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("RawOccupiedEntryMut") . field ("key" , self . key ()) . field ("value" , self . get ()) . finish () } }
};
}
