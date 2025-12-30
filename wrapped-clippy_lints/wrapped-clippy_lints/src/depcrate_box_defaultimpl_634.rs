// Generated macro for impl_634 (impl)
macro_rules! Depcrate_box_defaultimpl_634 {
() => {
// Module: crate::box_default
// Provides: {"impl_634"}
// Dependencies: {}
impl LateLintPass < '_ > for BoxDefault { fn check_expr (& mut self , cx : & LateContext < '_ > , expr : & Expr < '_ >) { if let ExprKind :: Call (box_new , [arg]) = expr . kind && let ExprKind :: Path (QPath :: TypeRelative (ty , seg)) = box_new . kind && seg . ident . name == sym :: new && ty . basic_res () . is_lang_item (cx , LangItem :: OwnedBox) && let ExprKind :: Call (arg_path , _) = arg . kind && ! expr . span . in_external_macro (cx . sess () . source_map ()) && (expr . span . eq_ctxt (arg . span) || is_local_vec_expn (cx , arg , expr)) && (is_plain_default (cx , arg_path) || (given_type (cx , expr) && is_default_equivalent (cx , arg))) { span_lint_and_sugg (cx , BOX_DEFAULT , expr . span , "`Box::new(_)` of default value" , "try" , "Box::default()" . into () , Applicability :: MachineApplicable ,) ; } } }
};
}
