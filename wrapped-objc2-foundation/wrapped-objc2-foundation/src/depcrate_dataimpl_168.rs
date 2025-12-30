// Generated macro for impl_168 (impl)
macro_rules! Depcrate_dataimpl_168 {
() => {
// Module: crate::data
// Provides: {"impl_168"}
// Dependencies: {}
# [cfg (feature = "block2")] impl RetainedFromIterator < u8 > for NSMutableData { fn retained_from_iter < I : IntoIterator < Item = u8 > > (iter : I) -> Retained < Self > { let vec = Vec :: from_iter (iter) ; Self :: from_vec (vec) } }
};
}
