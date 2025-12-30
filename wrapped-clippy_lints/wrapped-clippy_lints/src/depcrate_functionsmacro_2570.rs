// Generated macro for macro_2570 (macro)
macro_rules! Depcrate_functionsmacro_2570 {
() => {
// Module: crate::functions
// Provides: {"macro_2570"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for functions with too many parameters."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Functions with lots of parameters are considered bad"] # [doc = " style and reduce readability (“what does the 5th parameter mean?”). Consider"] # [doc = " grouping some parameters into a new type."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # struct Color;"] # [doc = " fn foo(x: u32, y: u32, name: &str, c: Color, w: f32, h: f32, a: f32, b: f32) {"] # [doc = "     // .."] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub TOO_MANY_ARGUMENTS , complexity , "functions with too many arguments" }
};
}
