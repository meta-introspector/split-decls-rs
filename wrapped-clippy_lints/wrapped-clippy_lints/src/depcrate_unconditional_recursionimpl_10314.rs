// Generated macro for impl_10314 (impl)
macro_rules! Depcrate_unconditional_recursionimpl_10314 {
() => {
// Module: crate::unconditional_recursion
// Provides: {"impl_10314"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for UnconditionalRecursion { fn check_fn (& mut self , cx : & LateContext < 'tcx > , kind : FnKind < 'tcx > , decl : & 'tcx FnDecl < 'tcx > , body : & 'tcx Body < 'tcx > , method_span : Span , method_def_id : LocalDefId ,) { if let FnKind :: Method (name , _) = kind && let expr = expr_or_init (cx , body . value) . peel_blocks () && ! has_conditional_return (body , expr) { match name . name { sym :: eq | sym :: ne => check_partial_eq (cx , method_span , method_def_id , name , expr) , sym :: to_string => check_to_string (cx , method_span , method_def_id , name , expr) , sym :: from => check_from (cx , method_span , method_def_id , expr) , _ => { } , } self . check_default_new (cx , decl , body , method_span , method_def_id) ; } } }
};
}
