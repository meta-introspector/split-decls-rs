// Generated macro for impl_2329 (impl)
macro_rules! Depcrate_setimpl_2329 {
() => {
// Module: crate::set
// Provides: {"impl_2329"}
// Dependencies: {}
# [cfg (feature = "NSEnumerator")] unsafe impl < ObjectType : Message > iter :: FastEnumerationHelper for NSMutableSet < ObjectType > { type Item = ObjectType ; # [inline] fn maybe_len (& self) -> Option < usize > { Some (self . len ()) } }
};
}
