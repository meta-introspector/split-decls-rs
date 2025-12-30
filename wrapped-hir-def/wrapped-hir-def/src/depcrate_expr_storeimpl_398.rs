// Generated macro for impl_398 (impl)
macro_rules! Depcrate_expr_storeimpl_398 {
() => {
// Module: crate::expr_store
// Provides: {"impl_398"}
// Dependencies: {}
impl Index < PathId > for ExpressionStore { type Output = Path ; # [inline] fn index (& self , index : PathId) -> & Self :: Output { let TypeRef :: Path (path) = & self [index . type_ref ()] else { unreachable ! ("`PathId` always points to `TypeRef::Path`") ; } ; path } }
};
}
