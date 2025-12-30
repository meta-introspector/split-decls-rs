// Generated macro for check_ty (function)
macro_rules! Depcrate_functions_ref_optioncheck_ty {
() => {
// Module: crate::functions::ref_option
// Provides: {"check_ty"}
// Dependencies: {}
fn check_ty < 'a > (cx : & LateContext < 'a > , param : & hir :: Ty < 'a > , param_ty : Ty < 'a > , fixes : & mut Vec < (Span , String) >) { if ! param . span . in_external_macro (cx . sess () . source_map ()) && let ty :: Ref (_ , opt_ty , Mutability :: Not) = param_ty . kind () && let Some (gen_ty) = option_arg_ty (cx , * opt_ty) && ! gen_ty . is_ref () && let hir :: TyKind :: Ref (lifetime , hir :: MutTy { ty , .. }) = param . kind && let hir :: TyKind :: Path (hir :: QPath :: Resolved (_ , path)) = ty . kind && let (Some (first) , Some (last)) = (path . segments . first () , path . segments . last ()) && let Some (hir :: GenericArgs { args : [hir :: GenericArg :: Type (opt_ty)] , .. }) = last . args && ! is_from_proc_macro (cx , param) { let lifetime = snippet (cx , lifetime . ident . span , "..") ; fixes . push ((param . span , format ! ("{}<&{lifetime}{}{}>" , snippet (cx , first . ident . span . to (last . ident . span) , "..") , if lifetime . is_empty () { "" } else { " " } , snippet (cx , opt_ty . span , "..")) ,)) ; } }
};
}
