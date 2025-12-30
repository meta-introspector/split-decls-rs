// Generated macro for impl_782 (impl)
macro_rules! Depcrate_provider_skeleton_referenceimpl_782 {
() => {
// Module: crate::provider::skeleton::reference
// Provides: {"impl_782"}
// Dependencies: {}
impl Skeleton { pub (crate) fn fields_iter (& self) -> impl Iterator < Item = & Field > { self . 0 . iter () } pub (crate) fn fields_len (& self) -> usize { self . 0 . len () } # [doc = " Return the underlying fields as a slice."] pub fn as_slice (& self) -> & [fields :: Field] { self . 0 . as_slice () } }
};
}
