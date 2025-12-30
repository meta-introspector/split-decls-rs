// Generated macro for impl_9236 (impl)
macro_rules! Depcrate_read_zero_byte_vecimpl_9236 {
() => {
// Module: crate::read_zero_byte_vec
// Provides: {"impl_9236"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for ReadVecVisitor < 'tcx > { fn visit_expr (& mut self , e : & 'tcx Expr < 'tcx >) { if let ExprKind :: MethodCall (path , receiver , args , _) = e . kind { let PathSegment { ident , .. } = * path ; match ident . name { sym :: read | sym :: read_exact => { let [arg] = args else { return } ; if let ExprKind :: AddrOf (_ , hir :: Mutability :: Mut , inner) = arg . kind && let ExprKind :: Path (QPath :: Resolved (None , inner_path)) = inner . kind && let [inner_seg] = inner_path . segments && let Res :: Local (res_id) = inner_seg . res && self . local_id == res_id { self . read_zero_expr = Some (e) ; return ; } } , sym :: resize => { if let ExprKind :: Path (QPath :: Resolved (_ , inner_path)) = receiver . kind && let Res :: Local (res_id) = inner_path . res && self . local_id == res_id { self . has_resize = true ; return ; } } , _ => { } , } } if ! self . has_resize && self . read_zero_expr . is_none () { walk_expr (self , e) ; } } }
};
}
