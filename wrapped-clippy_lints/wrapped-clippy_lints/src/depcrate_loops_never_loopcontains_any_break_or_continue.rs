// Generated macro for contains_any_break_or_continue (function)
macro_rules! Depcrate_loops_never_loopcontains_any_break_or_continue {
() => {
// Module: crate::loops::never_loop
// Provides: {"contains_any_break_or_continue"}
// Dependencies: {}
fn contains_any_break_or_continue (block : & Block < '_ >) -> bool { for_each_expr_without_closures (block , | e | match e . kind { ExprKind :: Break (..) | ExprKind :: Continue (..) => ControlFlow :: Break (()) , ExprKind :: InlineAsm (asm) if contains_label (asm) => ControlFlow :: Break (()) , ExprKind :: Loop (..) => ControlFlow :: Continue (Descend :: No) , _ => ControlFlow :: Continue (Descend :: Yes) , }) . is_some () }
};
}
