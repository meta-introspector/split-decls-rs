// Generated macro for impl_393 (impl)
macro_rules! Depcrate_expr_storeimpl_393 {
() => {
// Module: crate::expr_store
// Provides: {"impl_393"}
// Dependencies: {}
impl Index < PatId > for ExpressionStore { type Output = Pat ; # [inline] fn index (& self , pat : PatId) -> & Pat { & self . assert_expr_only () . pats [pat] } }
};
}
