// Generated macro for check_node (function)
macro_rules! Depcrate_utils_authorcheck_node {
() => {
// Module: crate::utils::author
// Provides: {"check_node"}
// Dependencies: {}
fn check_node (cx : & LateContext < '_ > , hir_id : HirId , f : impl Fn (& PrintVisitor < '_ , '_ >)) { if has_attr (cx , hir_id) { f (& PrintVisitor :: new (cx)) ; println ! ("{{") ; println ! ("    // report your lint here") ; println ! ("}}") ; } }
};
}
