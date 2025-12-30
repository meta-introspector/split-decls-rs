// Generated macro for impl_540 (impl)
macro_rules! Depcrate_infer_unify_keyimpl_540 {
() => {
// Module: crate::infer::unify_key
// Provides: {"impl_540"}
// Dependencies: {}
impl < 'tcx > UnifyKey for ConstVidKey < 'tcx > { type Value = ConstVariableValue < 'tcx > ; # [inline] fn index (& self) -> u32 { self . vid . as_u32 () } # [inline] fn from_index (i : u32) -> Self { ConstVidKey :: from (ty :: ConstVid :: from_u32 (i)) } fn tag () -> & 'static str { "ConstVidKey" } fn order_roots (a : Self , _ : & Self :: Value , b : Self , _ : & Self :: Value) -> Option < (Self , Self) > { if a . vid . as_u32 () < b . vid . as_u32 () { Some ((a , b)) } else { Some ((b , a)) } } }
};
}
