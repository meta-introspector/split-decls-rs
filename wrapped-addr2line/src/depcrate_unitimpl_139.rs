// Generated macro for impl_139 (impl)
macro_rules! Depcrate_unitimpl_139 {
() => {
// Module: crate::unit
// Provides: {"impl_139"}
// Dependencies: {}
# [cfg (feature = "fallible-iterator")] impl < 'ctx , R > fallible_iterator :: FallibleIterator for LocationRangeIter < 'ctx , R > where R : gimli :: Reader + 'ctx , { type Item = (u64 , u64 , Location < 'ctx >) ; type Error = Error ; # [inline] fn next (& mut self) -> Result < Option < Self :: Item > , Self :: Error > { self . next_loc () } }
};
}
