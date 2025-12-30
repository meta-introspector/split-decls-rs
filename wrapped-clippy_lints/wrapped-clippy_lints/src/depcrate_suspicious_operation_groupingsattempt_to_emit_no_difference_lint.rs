// Generated macro for attempt_to_emit_no_difference_lint (function)
macro_rules! Depcrate_suspicious_operation_groupingsattempt_to_emit_no_difference_lint {
() => {
// Module: crate::suspicious_operation_groupings
// Provides: {"attempt_to_emit_no_difference_lint"}
// Dependencies: {}
fn attempt_to_emit_no_difference_lint (cx : & EarlyContext < '_ > , binops : & [& BinaryOp < '_ >] , i : usize , expected_loc : IdentLocation ,) { if let Some (binop) = binops . get (i) . copied () { let mut applicability = Applicability :: MaybeIncorrect ; let old_left_ident = get_ident (binop . left , expected_loc) ; let old_right_ident = get_ident (binop . right , expected_loc) ; for b in skip_index (binops . iter () , i) { if let (Some (old_ident) , Some (new_ident)) = (old_left_ident , get_ident (b . left , expected_loc)) && old_ident != new_ident && let Some (sugg) = suggestion_with_swapped_ident (cx , binop . left , expected_loc , new_ident , & mut applicability) { emit_suggestion (cx , binop . span , replace_left_sugg (cx , binop , & sugg , & mut applicability) , applicability ,) ; return ; } if let (Some (old_ident) , Some (new_ident)) = (old_right_ident , get_ident (b . right , expected_loc)) && old_ident != new_ident && let Some (sugg) = suggestion_with_swapped_ident (cx , binop . right , expected_loc , new_ident , & mut applicability) { emit_suggestion (cx , binop . span , replace_right_sugg (cx , binop , & sugg , & mut applicability) , applicability ,) ; return ; } } } }
};
}
