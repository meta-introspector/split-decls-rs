// Generated macro for macro_502 (macro)
macro_rules! Depcrate_blocks_in_conditionsmacro_502 {
() => {
// Module: crate::blocks_in_conditions
// Provides: {"macro_502"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `if` and `match` conditions that use blocks containing an"] # [doc = " expression, statements or conditions that use closures with blocks."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Style, using blocks in the condition makes it hard to read."] # [doc = ""] # [doc = " ### Examples"] # [doc = " ```no_run"] # [doc = " # fn somefunc() -> bool { true };"] # [doc = " if { true } { /* ... */ }"] # [doc = ""] # [doc = " if { let x = somefunc(); x } { /* ... */ }"] # [doc = ""] # [doc = " match { let e = somefunc(); e } {"] # [doc = "     // ..."] # [doc = " #   _ => {}"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # fn somefunc() -> bool { true };"] # [doc = " if true { /* ... */ }"] # [doc = ""] # [doc = " let res = { let x = somefunc(); x };"] # [doc = " if res { /* ... */ }"] # [doc = ""] # [doc = " let res = { let e = somefunc(); e };"] # [doc = " match res {"] # [doc = "     // ..."] # [doc = " #   _ => {}"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.45.0"] pub BLOCKS_IN_CONDITIONS , style , "useless or complex blocks that can be eliminated in conditions" }
};
}
