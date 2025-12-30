// Generated macro for impl_2342 (impl)
macro_rules! Depcrate_setimpl_2342 {
() => {
// Module: crate::set
// Provides: {"impl_2342"}
// Dependencies: {}
impl < 'a , ObjectType : Message + 'a > RetainedFromIterator < & 'a ObjectType > for NSSet < ObjectType > { fn retained_from_iter < I : IntoIterator < Item = & 'a ObjectType > > (iter : I) -> Retained < Self > { let vec = Vec :: from_iter (iter) ; Self :: from_slice (& vec) } }
};
}
