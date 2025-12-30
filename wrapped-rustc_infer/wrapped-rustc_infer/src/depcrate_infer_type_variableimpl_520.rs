// Generated macro for impl_520 (impl)
macro_rules! Depcrate_infer_type_variableimpl_520 {
() => {
// Module: crate::infer::type_variable
// Provides: {"impl_520"}
// Dependencies: {}
impl ut :: UnifyKey for TyVidSubKey { type Value = () ; # [inline] fn index (& self) -> u32 { self . vid . as_u32 () } # [inline] fn from_index (i : u32) -> TyVidSubKey { TyVidSubKey { vid : ty :: TyVid :: from_u32 (i) } } fn tag () -> & 'static str { "TyVidSubKey" } }
};
}
