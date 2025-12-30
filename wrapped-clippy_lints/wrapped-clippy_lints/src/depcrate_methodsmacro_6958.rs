// Generated macro for macro_6958 (macro)
macro_rules! Depcrate_methodsmacro_6958 {
() => {
// Module: crate::methods
// Provides: {"macro_6958"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for consecutive calls to `str::replace` (2 or more)"] # [doc = " that can be collapsed into a single call."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Consecutive `str::replace` calls scan the string multiple times"] # [doc = " with repetitive code."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let hello = \"hesuo worpd\""] # [doc = "     .replace('s', \"l\")"] # [doc = "     .replace(\"u\", \"l\")"] # [doc = "     .replace('p', \"l\");"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let hello = \"hesuo worpd\".replace(['s', 'u', 'p'], \"l\");"] # [doc = " ```"] # [clippy :: version = "1.65.0"] pub COLLAPSIBLE_STR_REPLACE , perf , "collapse consecutive calls to str::replace (2 or more) into a single call" }
};
}
