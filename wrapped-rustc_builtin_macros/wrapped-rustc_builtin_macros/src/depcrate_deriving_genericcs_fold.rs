// Generated macro for cs_fold (function)
macro_rules! Depcrate_deriving_genericcs_fold {
() => {
// Module: crate::deriving::generic
// Provides: {"cs_fold"}
// Dependencies: {}
# [doc = " Folds over fields, combining the expressions for each field in a sequence."] # [doc = " Statics may not be folded over."] pub (crate) fn cs_fold < F > (use_foldl : bool , cx : & ExtCtxt < '_ > , trait_span : Span , substructure : & Substructure < '_ > , mut f : F ,) -> Box < Expr > where F : FnMut (& ExtCtxt < '_ > , CsFold < '_ >) -> Box < Expr > , { match substructure . fields { EnumMatching (.. , all_fields) | Struct (_ , all_fields) => { if all_fields . is_empty () { return f (cx , CsFold :: Fieldless) ; } let (base_field , rest) = if use_foldl { all_fields . split_first () . unwrap () } else { all_fields . split_last () . unwrap () } ; let base_expr = f (cx , CsFold :: Single (base_field)) ; let op = | old , field : & FieldInfo | { let new = f (cx , CsFold :: Single (field)) ; f (cx , CsFold :: Combine (field . span , old , new)) } ; if use_foldl { rest . iter () . fold (base_expr , op) } else { rest . iter () . rfold (base_expr , op) } } EnumDiscr (discr_field , match_expr) => { let discr_check_expr = f (cx , CsFold :: Single (discr_field)) ; if let Some (match_expr) = match_expr { if use_foldl { f (cx , CsFold :: Combine (trait_span , discr_check_expr , match_expr . clone ())) } else { f (cx , CsFold :: Combine (trait_span , match_expr . clone () , discr_check_expr)) } } else { discr_check_expr } } StaticEnum (..) | StaticStruct (..) => { cx . dcx () . span_bug (trait_span , "static function in `derive`") } AllFieldlessEnum (..) => cx . dcx () . span_bug (trait_span , "fieldless enum in `derive`") , } }
};
}
