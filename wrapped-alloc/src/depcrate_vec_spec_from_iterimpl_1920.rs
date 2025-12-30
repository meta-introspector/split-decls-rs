// Generated macro for impl_1920 (impl)
macro_rules! Depcrate_vec_spec_from_iterimpl_1920 {
() => {
// Module: crate::vec::spec_from_iter
// Provides: {"impl_1920"}
// Dependencies: {}
impl < T , I > SpecFromIter < T , I > for Vec < T > where I : Iterator < Item = T > , { # [track_caller] default fn from_iter (iterator : I) -> Self { SpecFromIterNested :: from_iter (iterator) } }
};
}
