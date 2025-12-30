// Generated macro for impl_96 (impl)
macro_rules! Depcrate_arrayimpl_96 {
() => {
// Module: crate::array
// Provides: {"impl_96"}
// Dependencies: {}
impl < ObjectType : Message > RetainedFromIterator < Retained < ObjectType > > for NSArray < ObjectType > { fn retained_from_iter < I : IntoIterator < Item = Retained < ObjectType > > > (iter : I) -> Retained < Self > { let vec = Vec :: from_iter (iter) ; Self :: from_retained_slice (& vec) } }
};
}
