// Generated macro for impl_854 (impl)
macro_rules! Depcrate_unordimpl_854 {
() => {
// Module: crate::unord
// Provides: {"impl_854"}
// Dependencies: {}
impl < 'a , T : Clone + 'a , I : Iterator < Item = & 'a T > > UnordItems < & 'a T , I > { # [inline] pub fn cloned (self) -> UnordItems < T , impl Iterator < Item = T > > { UnordItems (self . 0 . cloned ()) } }
};
}
