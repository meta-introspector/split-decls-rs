// Generated macro for impl_863 (impl)
macro_rules! Depcrate_unordimpl_863 {
() => {
// Module: crate::unord
// Provides: {"impl_863"}
// Dependencies: {}
impl < C : Extend < T > + UnordCollection , T > ExtendUnord < T > for C { # [inline] fn extend_unord < I : Iterator < Item = T > > (& mut self , items : UnordItems < T , I >) { self . extend (items . 0) } }
};
}
