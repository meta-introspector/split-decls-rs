// Generated macro for macro_10540 (macro)
macro_rules! Depcrate_unnecessary_map_on_constructormacro_10540 {
() => {
// Module: crate::unnecessary_map_on_constructor
// Provides: {"macro_10540"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Suggests removing the use of a `map()` (or `map_err()`) method when an `Option` or `Result`"] # [doc = " is being constructed."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It introduces unnecessary complexity. Instead, the function can be called before"] # [doc = " constructing the `Option` or `Result` from its return value."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " Some(4).map(i32::swap_bytes)"] # [doc = " # ;"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " Some(i32::swap_bytes(4))"] # [doc = " # ;"] # [doc = " ```"] # [clippy :: version = "1.74.0"] pub UNNECESSARY_MAP_ON_CONSTRUCTOR , complexity , "using `map`/`map_err` on `Option` or `Result` constructors" }
};
}
