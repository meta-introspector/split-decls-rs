// Generated macro for replace_right_sugg (function)
macro_rules! Depcrate_suspicious_operation_groupingsreplace_right_sugg {
() => {
// Module: crate::suspicious_operation_groupings
// Provides: {"replace_right_sugg"}
// Dependencies: {}
fn replace_right_sugg (cx : & EarlyContext < '_ > , binop : & BinaryOp < '_ > , right_suggestion : & str , applicability : & mut Applicability ,) -> String { format ! ("{} {} {right_suggestion}" , snippet_with_applicability (cx , binop . left . span , ".." , applicability) , binop . op . as_str () ,) }
};
}
