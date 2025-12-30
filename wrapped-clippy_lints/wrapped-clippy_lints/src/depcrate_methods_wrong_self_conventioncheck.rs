// Generated macro for check (function)
macro_rules! Depcrate_methods_wrong_self_conventioncheck {
() => {
// Module: crate::methods::wrong_self_convention
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , item_name : Symbol , self_ty : Ty < 'tcx > , first_arg_ty : Ty < 'tcx > , first_arg_span : Span , implements_trait : bool , is_trait_item : bool ,) { let item_name_str = item_name . as_str () ; if let Some ((conventions , self_kinds)) = & CONVENTIONS . iter () . find (| (convs , _) | { convs . iter () . all (| conv | conv . check (cx , self_ty , item_name_str , implements_trait , is_trait_item)) }) { if implements_trait && ! conventions . iter () . any (| conv | matches ! (conv , Convention :: IsSelfTypeCopy (_))) { return ; } if ! self_kinds . iter () . any (| k | k . matches (cx , self_ty , first_arg_ty)) { let suggestion = { if conventions . len () > 1 { let cut_ends_with_conv = conventions . iter () . any (| conv | matches ! (conv , Convention :: StartsWith (_))) && conventions . iter () . any (| conv | matches ! (conv , Convention :: NotEndsWith (_))) ; let s = conventions . iter () . filter (| conv | ! (cut_ends_with_conv && matches ! (conv , Convention :: NotEndsWith (_)))) . filter (| conv | ! matches ! (conv , Convention :: ImplementsTrait (_) | Convention :: IsTraitItem (_))) . format (" and ") ; format ! ("methods with the following characteristics: ({s})") } else { format ! ("methods called {}" , & conventions [0]) } } ; span_lint_and_help (cx , WRONG_SELF_CONVENTION , first_arg_span , format ! ("{suggestion} usually take {}" , self_kinds . iter () . map (| k | k . description ()) . format (" or ")) , None , "consider choosing a less ambiguous name" ,) ; } } }
};
}
