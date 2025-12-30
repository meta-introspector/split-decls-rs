// Generated macro for impl_889 (impl)
macro_rules! Depcrate_util_primitivesimpl_889 {
() => {
// Module: crate::util::primitives
// Provides: {"impl_889"}
// Dependencies: {}
impl Iterator for SmallIndexIter { type Item = SmallIndex ; fn next (& mut self) -> Option < SmallIndex > { if self . rng . start >= self . rng . end { return None ; } let next_id = self . rng . start + 1 ; let id = core :: mem :: replace (& mut self . rng . start , next_id) ; Some (SmallIndex :: new_unchecked (id)) } }
};
}
