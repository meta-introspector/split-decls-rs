// Generated macro for macro_4387 (macro)
macro_rules! Depcrate_manual_rotatemacro_4387 {
() => {
// Module: crate::manual_rotate
// Provides: {"macro_4387"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = ""] # [doc = " It detects manual bit rotations that could be rewritten using standard"] # [doc = " functions `rotate_left` or `rotate_right`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = ""] # [doc = " Calling the function better conveys the intent."] # [doc = ""] # [doc = " ### Known issues"] # [doc = ""] # [doc = " Currently, the lint only catches shifts by constant amount."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let x = 12345678_u32;"] # [doc = " let _ = (x >> 8) | (x << 24);"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let x = 12345678_u32;"] # [doc = " let _ = x.rotate_right(8);"] # [doc = " ```"] # [clippy :: version = "1.81.0"] pub MANUAL_ROTATE , style , "using bit shifts to rotate integers" }
};
}
