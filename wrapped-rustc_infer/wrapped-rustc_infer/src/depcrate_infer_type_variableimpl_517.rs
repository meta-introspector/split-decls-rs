// Generated macro for impl_517 (impl)
macro_rules! Depcrate_infer_type_variableimpl_517 {
() => {
// Module: crate::infer::type_variable
// Provides: {"impl_517"}
// Dependencies: {}
impl < 'tcx > ut :: UnifyKey for TyVidEqKey < 'tcx > { type Value = TypeVariableValue < 'tcx > ; # [inline (always)] fn index (& self) -> u32 { self . vid . as_u32 () } # [inline] fn from_index (i : u32) -> Self { TyVidEqKey :: from (ty :: TyVid :: from_u32 (i)) } fn tag () -> & 'static str { "TyVidEqKey" } fn order_roots (a : Self , _ : & Self :: Value , b : Self , _ : & Self :: Value) -> Option < (Self , Self) > { if a . vid . as_u32 () < b . vid . as_u32 () { Some ((a , b)) } else { Some ((b , a)) } } }
};
}
