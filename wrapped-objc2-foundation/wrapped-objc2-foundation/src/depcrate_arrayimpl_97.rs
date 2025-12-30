// Generated macro for impl_97 (impl)
macro_rules! Depcrate_arrayimpl_97 {
() => {
// Module: crate::array
// Provides: {"impl_97"}
// Dependencies: {}
impl < 'a , ObjectType : Message + 'a > RetainedFromIterator < & 'a ObjectType > for NSMutableArray < ObjectType > { fn retained_from_iter < I : IntoIterator < Item = & 'a ObjectType > > (iter : I) -> Retained < Self > { let vec = Vec :: from_iter (iter) ; Self :: from_slice (& vec) } }
};
}
