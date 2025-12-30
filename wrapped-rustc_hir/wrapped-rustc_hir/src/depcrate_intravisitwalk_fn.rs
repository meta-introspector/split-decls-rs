// Generated macro for walk_fn (function)
macro_rules! Depcrate_intravisitwalk_fn {
() => {
// Module: crate::intravisit
// Provides: {"walk_fn"}
// Dependencies: {}
pub fn walk_fn < 'v , V : Visitor < 'v > > (visitor : & mut V , function_kind : FnKind < 'v > , function_declaration : & 'v FnDecl < 'v > , body_id : BodyId , _ : LocalDefId ,) -> V :: Result { try_visit ! (visitor . visit_fn_decl (function_declaration)) ; try_visit ! (walk_fn_kind (visitor , function_kind)) ; visitor . visit_nested_body (body_id) }
};
}
