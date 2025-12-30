// Generated macro for impl_9825 (impl)
macro_rules! Depcrate_single_component_path_importsimpl_9825 {
() => {
// Module: crate::single_component_path_imports
// Provides: {"impl_9825"}
// Dependencies: {}
impl Visitor < '_ > for ImportUsageVisitor { fn visit_expr (& mut self , expr : & Expr) { if let ExprKind :: Path (_ , path) = & expr . kind && path . segments . len () > 1 && path . segments [0] . ident . name == kw :: SelfLower { self . imports_referenced_with_self . push (path . segments [1] . ident . name) ; } walk_expr (self , expr) ; } fn visit_ty (& mut self , ty : & Ty) { if let TyKind :: Path (_ , path) = & ty . kind && path . segments . len () > 1 && path . segments [0] . ident . name == kw :: SelfLower { self . imports_referenced_with_self . push (path . segments [1] . ident . name) ; } } }
};
}
