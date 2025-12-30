// Generated macro for impl_9450 (impl)
macro_rules! Depcrate_ref_option_refimpl_9450 {
() => {
// Module: crate::ref_option_ref
// Provides: {"impl_9450"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for RefOptionRef { fn check_ty (& mut self , cx : & LateContext < 'tcx > , ty : & 'tcx Ty < 'tcx , AmbigArg >) { if let TyKind :: Ref (_ , ref mut_ty) = ty . kind && mut_ty . mutbl == Mutability :: Not && let TyKind :: Path (qpath) = & mut_ty . ty . kind && let last = last_path_segment (qpath) && let Some (def_id) = last . res . opt_def_id () && cx . tcx . is_diagnostic_item (sym :: Option , def_id) && let Some (params) = last_path_segment (qpath) . args && params . parenthesized == GenericArgsParentheses :: No && let Some (inner_ty) = params . args . iter () . find_map (| arg | match arg { GenericArg :: Type (inner_ty) => Some (inner_ty) , _ => None , }) && let TyKind :: Ref (_ , ref inner_mut_ty) = inner_ty . kind && inner_mut_ty . mutbl == Mutability :: Not { span_lint_and_sugg (cx , REF_OPTION_REF , ty . span , "since `&` implements the `Copy` trait, `&Option<&T>` can be simplified to `Option<&T>`" , "try" , format ! ("Option<{}>" , & snippet (cx , inner_ty . span , "..")) , Applicability :: MaybeIncorrect ,) ; } } }
};
}
