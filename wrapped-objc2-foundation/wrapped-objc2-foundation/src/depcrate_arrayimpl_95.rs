// Generated macro for impl_95 (impl)
macro_rules! Depcrate_arrayimpl_95 {
() => {
// Module: crate::array
// Provides: {"impl_95"}
// Dependencies: {}
impl < 'a , ObjectType : Message + 'a > RetainedFromIterator < & 'a ObjectType > for NSArray < ObjectType > { fn retained_from_iter < I : IntoIterator < Item = & 'a ObjectType > > (iter : I) -> Retained < Self > { let vec = Vec :: from_iter (iter) ; Self :: from_slice (& vec) } }
};
}
