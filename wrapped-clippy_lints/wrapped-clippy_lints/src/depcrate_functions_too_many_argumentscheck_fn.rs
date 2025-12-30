// Generated macro for check_fn (function)
macro_rules! Depcrate_functions_too_many_argumentscheck_fn {
() => {
// Module: crate::functions::too_many_arguments
// Provides: {"check_fn"}
// Dependencies: {}
pub (super) fn check_fn (cx : & LateContext < '_ > , kind : FnKind < '_ > , decl : & hir :: FnDecl < '_ > , hir_id : hir :: HirId , def_id : LocalDefId , too_many_arguments_threshold : u64 ,) { if ! is_trait_impl_item (cx , hir_id) && kind . header () . is_some_and (| header | header . abi == ExternAbi :: Rust) { check_arg_number (cx , decl , cx . tcx . def_span (def_id) , too_many_arguments_threshold) ; } }
};
}
