// Generated macro for impl_394 (impl)
macro_rules! Depcrate_expr_storeimpl_394 {
() => {
// Module: crate::expr_store
// Provides: {"impl_394"}
// Dependencies: {}
impl Index < LabelId > for ExpressionStore { type Output = Label ; # [inline] fn index (& self , label : LabelId) -> & Label { & self . assert_expr_only () . labels [label] } }
};
}
