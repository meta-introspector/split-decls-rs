// Generated macro for impl_395 (impl)
macro_rules! Depcrate_expr_storeimpl_395 {
() => {
// Module: crate::expr_store
// Provides: {"impl_395"}
// Dependencies: {}
impl Index < BindingId > for ExpressionStore { type Output = Binding ; # [inline] fn index (& self , b : BindingId) -> & Binding { & self . assert_expr_only () . bindings [b] } }
};
}
