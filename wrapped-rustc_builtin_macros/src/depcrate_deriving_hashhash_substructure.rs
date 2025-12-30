// Generated macro for hash_substructure (function)
macro_rules! Depcrate_deriving_hashhash_substructure {
() => {
// Module: crate::deriving::hash
// Provides: {"hash_substructure"}
// Dependencies: {}
fn hash_substructure (cx : & ExtCtxt < '_ > , trait_span : Span , substr : & Substructure < '_ >) -> BlockOrExpr { let [state_expr] = substr . nonselflike_args else { cx . dcx () . span_bug (trait_span , "incorrect number of arguments in `derive(Hash)`") ; } ; let call_hash = | span , expr | { let hash_path = { let strs = cx . std_path (& [sym :: hash , sym :: Hash , sym :: hash]) ; cx . expr_path (cx . path_global (span , strs)) } ; let expr = cx . expr_call (span , hash_path , thin_vec ! [expr , state_expr . clone ()]) ; cx . stmt_expr (expr) } ; let (stmts , match_expr) = match substr . fields { Struct (_ , fields) | EnumMatching (.. , fields) => { let stmts = fields . iter () . map (| field | call_hash (field . span , field . self_expr . clone ())) . collect () ; (stmts , None) } EnumDiscr (discr_field , match_expr) => { assert ! (discr_field . other_selflike_exprs . is_empty ()) ; let stmts = thin_vec ! [call_hash (discr_field . span , discr_field . self_expr . clone ())] ; (stmts , match_expr . clone ()) } _ => cx . dcx () . span_bug (trait_span , "impossible substructure in `derive(Hash)`") , } ; BlockOrExpr :: new_mixed (stmts , match_expr) }
};
}
