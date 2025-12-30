// Generated macro for pat_and_expr_can_be_question_mark (function)
macro_rules! Depcratepat_and_expr_can_be_question_mark {
() => {
// Module: crate
// Provides: {"pat_and_expr_can_be_question_mark"}
// Dependencies: {}
# [doc = " Returns whether the given let pattern and else body can be turned into the `?` operator"] # [doc = ""] # [doc = " For this example:"] # [doc = " ```ignore"] # [doc = " let FooBar { a, b } = if let Some(a) = ex { a } else { return None };"] # [doc = " ```"] # [doc = " We get as parameters:"] # [doc = " ```ignore"] # [doc = " pat: Some(a)"] # [doc = " else_body: return None"] # [doc = " ```"] # [doc = ""] # [doc = " And for this example:"] # [doc = " ```ignore"] # [doc = " let Some(FooBar { a, b }) = ex else { return None };"] # [doc = " ```"] # [doc = " We get as parameters:"] # [doc = " ```ignore"] # [doc = " pat: Some(FooBar { a, b })"] # [doc = " else_body: return None"] # [doc = " ```"] # [doc = ""] # [doc = " We output `Some(a)` in the first instance, and `Some(FooBar { a, b })` in the second, because"] # [doc = " the `?` operator is applicable here. Callers have to check whether we are in a constant or not."] pub fn pat_and_expr_can_be_question_mark < 'a , 'hir > (cx : & LateContext < '_ > , pat : & 'a Pat < 'hir > , else_body : & Expr < '_ > ,) -> Option < & 'a Pat < 'hir > > { if let Some ([inner_pat]) = as_some_pattern (cx , pat) && ! is_refutable (cx , inner_pat) && let else_body = peel_blocks (else_body) && let ExprKind :: Ret (Some (ret_val)) = else_body . kind && let ExprKind :: Path (ret_path) = ret_val . kind && cx . qpath_res (& ret_path , ret_val . hir_id) . ctor_parent (cx) . is_lang_item (cx , OptionNone) { Some (inner_pat) } else { None } }
};
}
