// Generated macro for macro_7003 (macro)
macro_rules! Depcrate_methodsmacro_7003 {
() => {
// Module: crate::methods
// Provides: {"macro_7003"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `filter_map` calls that could be replaced by `filter` or `map`."] # [doc = " More specifically it checks if the closure provided is only performing one of the"] # [doc = " filter or map operations and suggests the appropriate option."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Complexity. The intent is also clearer if only a single"] # [doc = " operation is being performed."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let _ = (0..3).filter_map(|x| if x > 2 { Some(x) } else { None });"] # [doc = ""] # [doc = " // As there is no transformation of the argument this could be written as:"] # [doc = " let _ = (0..3).filter(|&x| x > 2);"] # [doc = " ```"] # [doc = ""] # [doc = " ```no_run"] # [doc = " let _ = (0..4).filter_map(|x| Some(x + 1));"] # [doc = ""] # [doc = " // As there is no conditional check on the argument this could be written as:"] # [doc = " let _ = (0..4).map(|x| x + 1);"] # [doc = " ```"] # [clippy :: version = "1.31.0"] pub UNNECESSARY_FILTER_MAP , complexity , "using `filter_map` when a more succinct alternative exists" }
};
}
