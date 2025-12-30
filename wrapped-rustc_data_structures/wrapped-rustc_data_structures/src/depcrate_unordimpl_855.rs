// Generated macro for impl_855 (impl)
macro_rules! Depcrate_unordimpl_855 {
() => {
// Module: crate::unord
// Provides: {"impl_855"}
// Dependencies: {}
impl < 'a , T : Copy + 'a , I : Iterator < Item = & 'a T > > UnordItems < & 'a T , I > { # [inline] pub fn copied (self) -> UnordItems < T , impl Iterator < Item = T > > { UnordItems (self . 0 . copied ()) } }
};
}
