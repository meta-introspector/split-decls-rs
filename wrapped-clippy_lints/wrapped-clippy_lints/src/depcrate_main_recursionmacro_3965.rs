// Generated macro for macro_3965 (macro)
macro_rules! Depcrate_main_recursionmacro_3965 {
() => {
// Module: crate::main_recursion
// Provides: {"macro_3965"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for recursion using the entrypoint."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Apart from special setups (which we could detect following attributes like #![no_std]),"] # [doc = " recursing into main() seems like an unintuitive anti-pattern we should be able to detect."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn main() {"] # [doc = "     main();"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.38.0"] pub MAIN_RECURSION , style , "recursion using the entrypoint" }
};
}
