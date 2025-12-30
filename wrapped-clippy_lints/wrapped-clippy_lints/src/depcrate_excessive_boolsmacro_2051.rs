// Generated macro for macro_2051 (macro)
macro_rules! Depcrate_excessive_boolsmacro_2051 {
() => {
// Module: crate::excessive_bools
// Provides: {"macro_2051"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for excessive"] # [doc = " use of bools in structs."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Excessive bools in a struct is often a sign that"] # [doc = " the type is being used to represent a state"] # [doc = " machine, which is much better implemented as an"] # [doc = " enum."] # [doc = ""] # [doc = " The reason an enum is better for state machines"] # [doc = " over structs is that enums more easily forbid"] # [doc = " invalid states."] # [doc = ""] # [doc = " Structs with too many booleans may benefit from refactoring"] # [doc = " into multi variant enums for better readability and API."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " struct S {"] # [doc = "     is_pending: bool,"] # [doc = "     is_processing: bool,"] # [doc = "     is_finished: bool,"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " enum S {"] # [doc = "     Pending,"] # [doc = "     Processing,"] # [doc = "     Finished,"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.43.0"] pub STRUCT_EXCESSIVE_BOOLS , pedantic , "using too many bools in a struct" }
};
}
