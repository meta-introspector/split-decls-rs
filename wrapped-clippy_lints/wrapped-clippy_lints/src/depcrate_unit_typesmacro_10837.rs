// Generated macro for macro_10837 (macro)
macro_rules! Depcrate_unit_typesmacro_10837 {
() => {
// Module: crate::unit_types
// Provides: {"macro_10837"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for binding a unit value."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " A unit value cannot usefully be used anywhere. So"] # [doc = " binding one is kind of pointless."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let x = {"] # [doc = "     1;"] # [doc = " };"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub LET_UNIT_VALUE , style , "creating a `let` binding to a value of unit type, which usually can't be used afterwards" }
};
}
