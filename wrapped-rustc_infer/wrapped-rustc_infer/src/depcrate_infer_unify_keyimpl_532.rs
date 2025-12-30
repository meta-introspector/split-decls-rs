// Generated macro for impl_532 (impl)
macro_rules! Depcrate_infer_unify_keyimpl_532 {
() => {
// Module: crate::infer::unify_key
// Provides: {"impl_532"}
// Dependencies: {}
impl < 'tcx > UnifyKey for RegionVidKey < 'tcx > { type Value = RegionVariableValue < 'tcx > ; # [inline] fn index (& self) -> u32 { self . vid . as_u32 () } # [inline] fn from_index (i : u32) -> Self { RegionVidKey :: from (ty :: RegionVid :: from_u32 (i)) } fn tag () -> & 'static str { "RegionVidKey" } }
};
}
