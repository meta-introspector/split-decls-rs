// Generated macro for macro_8755 (macro)
macro_rules! Depcrate_operatorsmacro_8755 {
() => {
// Module: crate::operators
// Provides: {"macro_8755"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for explicit self-assignments."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Self-assignments are redundant and unlikely to be"] # [doc = " intentional."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " If expression contains any deref coercions or"] # [doc = " indexing operations they are assumed not to have any side effects."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " struct Event {"] # [doc = "     x: i32,"] # [doc = " }"] # [doc = ""] # [doc = " fn copy_position(a: &mut Event, b: &Event) {"] # [doc = "     a.x = a.x;"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Should be:"] # [doc = " ```no_run"] # [doc = " struct Event {"] # [doc = "     x: i32,"] # [doc = " }"] # [doc = ""] # [doc = " fn copy_position(a: &mut Event, b: &Event) {"] # [doc = "     a.x = b.x;"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.48.0"] pub SELF_ASSIGNMENT , correctness , "explicit self-assignment" }
};
}
