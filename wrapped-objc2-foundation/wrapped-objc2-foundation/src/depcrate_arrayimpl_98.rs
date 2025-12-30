// Generated macro for impl_98 (impl)
macro_rules! Depcrate_arrayimpl_98 {
() => {
// Module: crate::array
// Provides: {"impl_98"}
// Dependencies: {}
impl < ObjectType : Message > RetainedFromIterator < Retained < ObjectType > > for NSMutableArray < ObjectType > { fn retained_from_iter < I : IntoIterator < Item = Retained < ObjectType > > > (iter : I) -> Retained < Self > { let vec = Vec :: from_iter (iter) ; Self :: from_retained_slice (& vec) } }
};
}
