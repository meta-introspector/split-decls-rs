// Generated macro for extract_fn_extra_field (function)
macro_rules! Depcrate_parseextract_fn_extra_field {
() => {
// Module: crate::parse
// Provides: {"extract_fn_extra_field"}
// Dependencies: {}
fn extract_fn_extra_field (expr : Expr) -> syn :: Result < BTreeMap < Ident , Expr > > { let Expr :: Match (mexpr) = expr else { let e = syn :: Error :: new (expr . span () , "`fn_extra` expects a match expression") ; return Err (e) ; } ; let ExprMatch { attrs , match_token : _ , expr , brace_token : _ , arms , } = mexpr ; expect_empty_attrs (& attrs) ? ; let match_on = expect_ident (* expr) ? ; if match_on != "MACRO_FN_NAME" { let e = syn :: Error :: new (match_on . span () , "only allowed to match on `MACRO_FN_NAME`") ; return Err (e) ; } let mut res = BTreeMap :: new () ; for arm in arms { let Arm { attrs , pat , guard , fat_arrow_token : _ , body , comma : _ , } = arm ; expect_empty_attrs (& attrs) ? ; let keys = match pat { syn :: Pat :: Wild (w) => vec ! [Ident :: new ("_" , w . span ())] , _ => Parser :: parse2 (parse_ident_pat , pat . into_token_stream ()) ? , } ; if let Some (guard) = guard { let e = syn :: Error :: new (guard . 0 . span () , "no guards allowed in this position") ; return Err (e) ; } for key in keys { let inserted = res . insert (key . clone () , * body . clone ()) ; if inserted . is_some () { let e = syn :: Error :: new (key . span () , format ! ("key `{key}` specified twice")) ; return Err (e) ; } } } Ok (res) }
};
}
