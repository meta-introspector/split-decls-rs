// Generated macro for macro_1082 (macro)
macro_rules! Depcrate_castsmacro_1082 {
() => {
// Module: crate::casts
// Provides: {"macro_1082"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for casts to the same type, casts of int literals to integer"] # [doc = " types, casts of float literals to float types, and casts between raw"] # [doc = " pointers that don't change type or constness."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It's just unnecessary."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " When the expression on the left is a function call, the lint considers"] # [doc = " the return type to be a type alias if it's aliased through a `use`"] # [doc = " statement (like `use std::io::Result as IoResult`). It will not lint"] # [doc = " such cases."] # [doc = ""] # [doc = " This check will only work on primitive types without any intermediate"] # [doc = " references: raw pointers and trait objects may or may not work."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let _ = 2i32 as i32;"] # [doc = " let _ = 0.5 as f32;"] # [doc = " ```"] # [doc = ""] # [doc = " Better:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " let _ = 2_i32;"] # [doc = " let _ = 0.5_f32;"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub UNNECESSARY_CAST , complexity , "cast to the same type, e.g., `x as i32` where `x: i32`" }
};
}
