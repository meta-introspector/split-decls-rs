// Generated macro for check_arm_is_none_or_err (function)
macro_rules! Depcrate_question_markcheck_arm_is_none_or_err {
() => {
// Module: crate::question_mark
// Provides: {"check_arm_is_none_or_err"}
// Dependencies: {}
fn check_arm_is_none_or_err < 'tcx > (cx : & LateContext < 'tcx > , mode : TryMode , arm : & Arm < 'tcx >) -> bool { if arm . guard . is_some () { return false ; } let arm_body = peel_blocks (arm . body) ; match mode { TryMode :: Result => { if let Some (ok_pat) = extract_ctor_call (cx , ResultErr , arm . pat) && let Some (ok_val) = extract_binding_pat (ok_pat) && let ExprKind :: Ret (Some (wrapped_ret_expr)) = arm_body . kind && let ExprKind :: Call (ok_ctor , [ret_expr]) = wrapped_ret_expr . kind && ok_ctor . res (cx) . ctor_parent (cx) . is_lang_item (cx , ResultErr) && is_local_or_local_into (cx , ret_expr , ok_val) { true } else { false } } , TryMode :: Option => { if arm . pat . res (cx) . ctor_parent (cx) . is_lang_item (cx , OptionNone) && let ExprKind :: Ret (Some (ret_expr)) = arm_body . kind && ret_expr . res (cx) . ctor_parent (cx) . is_lang_item (cx , OptionNone) && ! ret_expr . span . from_expansion () { true } else { false } } , } }
};
}
