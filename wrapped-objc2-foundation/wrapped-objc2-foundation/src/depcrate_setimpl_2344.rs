// Generated macro for impl_2344 (impl)
macro_rules! Depcrate_setimpl_2344 {
() => {
// Module: crate::set
// Provides: {"impl_2344"}
// Dependencies: {}
impl < 'a , ObjectType : Message + 'a > RetainedFromIterator < & 'a ObjectType > for NSMutableSet < ObjectType > { fn retained_from_iter < I : IntoIterator < Item = & 'a ObjectType > > (iter : I) -> Retained < Self > { let vec = Vec :: from_iter (iter) ; Self :: from_slice (& vec) } }
};
}
