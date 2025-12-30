// Generated macro for is_early_return (function)
macro_rules! Depcrate_question_markis_early_return {
() => {
// Module: crate::question_mark
// Provides: {"is_early_return"}
// Dependencies: {}
fn is_early_return (smbl : Symbol , cx : & LateContext < '_ > , if_block : & IfBlockType < '_ >) -> bool { match * if_block { IfBlockType :: IfIs (caller , caller_ty , call_sym , if_then) => { caller_ty . is_diag_item (cx , smbl) && expr_return_none_or_err (smbl , cx , if_then , caller , None) && match smbl { sym :: Option => call_sym == sym :: is_none , sym :: Result => call_sym == sym :: is_err , _ => false , } } , IfBlockType :: IfLet (res , let_expr_ty , let_pat_sym , let_expr , if_then , if_else) => { let_expr_ty . is_diag_item (cx , smbl) && match smbl { sym :: Option => { res . ctor_parent (cx) . is_lang_item (cx , OptionSome) && if_else . is_some () && expr_return_none_or_err (smbl , cx , if_else . unwrap () , let_expr , None) } , sym :: Result => { (res . ctor_parent (cx) . is_lang_item (cx , ResultOk) && if_else . is_some () && expr_return_none_or_err (smbl , cx , if_else . unwrap () , let_expr , Some (let_pat_sym))) || res . ctor_parent (cx) . is_lang_item (cx , ResultErr) && expr_return_none_or_err (smbl , cx , if_then , let_expr , Some (let_pat_sym)) && if_else . is_none () } , _ => false , } } , } }
};
}
