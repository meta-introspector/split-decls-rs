// Generated macro for impl_2345 (impl)
macro_rules! Depcrate_setimpl_2345 {
() => {
// Module: crate::set
// Provides: {"impl_2345"}
// Dependencies: {}
impl < ObjectType : Message > RetainedFromIterator < Retained < ObjectType > > for NSMutableSet < ObjectType > { fn retained_from_iter < I : IntoIterator < Item = Retained < ObjectType > > > (iter : I) -> Retained < Self > { let vec = Vec :: from_iter (iter) ; Self :: from_retained_slice (& vec) } }
};
}
