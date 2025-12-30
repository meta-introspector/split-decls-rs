// Generated macro for replace_left_sugg (function)
macro_rules! Depcrate_suspicious_operation_groupingsreplace_left_sugg {
() => {
// Module: crate::suspicious_operation_groupings
// Provides: {"replace_left_sugg"}
// Dependencies: {}
fn replace_left_sugg (cx : & EarlyContext < '_ > , binop : & BinaryOp < '_ > , left_suggestion : & str , applicability : & mut Applicability ,) -> String { format ! ("{left_suggestion} {} {}" , binop . op . as_str () , snippet_with_applicability (cx , binop . right . span , ".." , applicability) ,) }
};
}
