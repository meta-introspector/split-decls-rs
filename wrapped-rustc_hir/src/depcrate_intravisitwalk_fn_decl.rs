// Generated macro for walk_fn_decl (function)
macro_rules! Depcrate_intravisitwalk_fn_decl {
() => {
// Module: crate::intravisit
// Provides: {"walk_fn_decl"}
// Dependencies: {}
pub fn walk_fn_decl < 'v , V : Visitor < 'v > > (visitor : & mut V , function_declaration : & 'v FnDecl < 'v > ,) -> V :: Result { let FnDecl { inputs , output , c_variadic : _ , implicit_self : _ , lifetime_elision_allowed : _ } = function_declaration ; walk_list ! (visitor , visit_ty_unambig , * inputs) ; visitor . visit_fn_ret_ty (output) }
};
}
