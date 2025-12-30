// Generated macro for macro_8934 (macro)
macro_rules! Depcrate_rangesmacro_8934 {
() => {
// Module: crate::ranges
// Provides: {"macro_8934"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for inclusive ranges where 1 is subtracted from"] # [doc = " the upper bound, e.g., `x..=(y-1)`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The code is more readable with an exclusive range"] # [doc = " like `x..y`."] # [doc = ""] # [doc = " ### Limitations"] # [doc = " The lint is conservative and will trigger only when switching"] # [doc = " from an inclusive to an exclusive range is provably safe from"] # [doc = " a typing point of view. This corresponds to situations where"] # [doc = " the range is used as an iterator, or for indexing."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let x = 0;"] # [doc = " # let y = 1;"] # [doc = " for i in x..=(y-1) {"] # [doc = "     // .."] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let x = 0;"] # [doc = " # let y = 1;"] # [doc = " for i in x..y {"] # [doc = "     // .."] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub RANGE_MINUS_ONE , pedantic , "`x..=(y-1)` reads better as `x..y`" }
};
}
