// Generated macro for raw_ptr_arg (function)
macro_rules! Depcrate_functions_not_unsafe_ptr_arg_derefraw_ptr_arg {
() => {
// Module: crate::functions::not_unsafe_ptr_arg_deref
// Provides: {"raw_ptr_arg"}
// Dependencies: {}
fn raw_ptr_arg (cx : & LateContext < '_ > , arg : & hir :: Param < '_ >) -> Option < HirId > { if let (& hir :: PatKind :: Binding (_ , id , _ , _) , Some (& ty :: RawPtr (_ , _))) = (& arg . pat . kind , cx . maybe_typeck_results () . map (| typeck_results | typeck_results . pat_ty (arg . pat) . kind ()) ,) { Some (id) } else { None } }
};
}
