// Generated macro for check (function)
macro_rules! Depcrate_types_borrowed_boxcheck {
() => {
// Module: crate::types::borrowed_box
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , hir_ty : & hir :: Ty < '_ > , lt : & Lifetime , mut_ty : & MutTy < '_ >) -> bool { match mut_ty . ty . kind { TyKind :: Path (ref qpath) => { let hir_id = mut_ty . ty . hir_id ; let def = cx . qpath_res (qpath , hir_id) ; if let Some (def_id) = def . opt_def_id () && Some (def_id) == cx . tcx . lang_items () . owned_box () && let QPath :: Resolved (None , path) = * qpath && let [ref bx] = * path . segments && let Some (params) = bx . args && params . parenthesized == hir :: GenericArgsParentheses :: No && let Some (inner) = params . args . iter () . find_map (| arg | match arg { GenericArg :: Type (ty) => Some (ty) , _ => None , }) { if is_any_trait (cx , inner . as_unambig_ty ()) { return false ; } let ltopt = if lt . is_anonymous () { String :: new () } else { format ! ("{} " , lt . ident) } ; if mut_ty . mutbl == Mutability :: Mut { return false ; } let inner_snippet = snippet (cx , inner . span , "..") ; let suggestion = match & inner . kind { TyKind :: TraitObject (bounds , lt_bound) if bounds . len () > 1 || ! lt_bound . is_elided () => { format ! ("&{ltopt}({inner_snippet})") } , TyKind :: Path (qpath) if get_bounds_if_impl_trait (cx , qpath , inner . hir_id) . is_some_and (| bounds | bounds . len () > 1) => { format ! ("&{ltopt}({inner_snippet})") } , _ => format ! ("&{ltopt}{inner_snippet}") , } ; span_lint_and_sugg (cx , BORROWED_BOX , hir_ty . span , "you seem to be trying to use `&Box<T>`. Consider using just `&T`" , "try" , suggestion , Applicability :: Unspecified ,) ; return true ; } false } , _ => false , } }
};
}
