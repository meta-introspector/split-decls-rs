// Generated macro for macro_7033 (macro)
macro_rules! Depcrate_methodsmacro_7033 {
() => {
// Module: crate::methods
// Provides: {"macro_7033"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `replace` statements which have no effect."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It's either a mistake or confusing."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " \"1234\".replace(\"12\", \"12\");"] # [doc = " \"1234\".replacen(\"12\", \"12\", 1);"] # [doc = " ```"] # [clippy :: version = "1.63.0"] pub NO_EFFECT_REPLACE , suspicious , "replace with no effect" }
};
}
