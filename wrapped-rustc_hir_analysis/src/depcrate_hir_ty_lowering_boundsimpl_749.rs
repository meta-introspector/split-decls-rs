// Generated macro for impl_749 (impl)
macro_rules! Depcrate_hir_ty_lowering_boundsimpl_749 {
() => {
// Module: crate::hir_ty_lowering::bounds
// Provides: {"impl_749"}
// Dependencies: {}
impl CollectedSizednessBounds { # [doc = " Returns `true` if any of `Trait`, `?Trait` or `!Trait` were encountered for `Sized`,"] # [doc = " `MetaSized` or `PointeeSized`."] fn any (& self) -> bool { self . sized . any () || self . meta_sized . any () || self . pointee_sized . any () } }
};
}
