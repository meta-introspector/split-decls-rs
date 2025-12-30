// Generated macro for impl_2328 (impl)
macro_rules! Depcrate_setimpl_2328 {
() => {
// Module: crate::set
// Provides: {"impl_2328"}
// Dependencies: {}
# [cfg (feature = "NSEnumerator")] unsafe impl < ObjectType : Message > iter :: FastEnumerationHelper for NSSet < ObjectType > { type Item = ObjectType ; # [inline] fn maybe_len (& self) -> Option < usize > { Some (self . len ()) } }
};
}
