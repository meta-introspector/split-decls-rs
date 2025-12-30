// Generated macro for macro_8752 (macro)
macro_rules! Depcrate_operatorsmacro_8752 {
() => {
// Module: crate::operators
// Provides: {"macro_8752"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for getting the remainder of integer division by one or minus"] # [doc = " one."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The result for a divisor of one can only ever be zero; for"] # [doc = " minus one it can cause panic/overflow (if the left operand is the minimal value of"] # [doc = " the respective integer type) or results in zero. No one will write such code"] # [doc = " deliberately, unless trying to win an Underhanded Rust Contest. Even for that"] # [doc = " contest, it's probably a bad idea. Use something more underhanded."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let x = 1;"] # [doc = " let a = x % 1;"] # [doc = " let a = x % -1;"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub MODULO_ONE , correctness , "taking an integer modulo +/-1, which can either panic/overflow or always returns 0" }
};
}
