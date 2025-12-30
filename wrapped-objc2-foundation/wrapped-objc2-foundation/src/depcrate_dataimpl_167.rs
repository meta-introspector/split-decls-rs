// Generated macro for impl_167 (impl)
macro_rules! Depcrate_dataimpl_167 {
() => {
// Module: crate::data
// Provides: {"impl_167"}
// Dependencies: {}
# [cfg (feature = "block2")] impl RetainedFromIterator < u8 > for NSData { fn retained_from_iter < I : IntoIterator < Item = u8 > > (iter : I) -> Retained < Self > { let vec = Vec :: from_iter (iter) ; Self :: from_vec (vec) } }
};
}
