// Generated macro for macro_7004 (macro)
macro_rules! Depcrate_methodsmacro_7004 {
() => {
// Module: crate::methods
// Provides: {"macro_7004"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `find_map` calls that could be replaced by `find` or `map`. More"] # [doc = " specifically it checks if the closure provided is only performing one of the"] # [doc = " find or map operations and suggests the appropriate option."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Complexity. The intent is also clearer if only a single"] # [doc = " operation is being performed."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let _ = (0..3).find_map(|x| if x > 2 { Some(x) } else { None });"] # [doc = ""] # [doc = " // As there is no transformation of the argument this could be written as:"] # [doc = " let _ = (0..3).find(|&x| x > 2);"] # [doc = " ```"] # [doc = ""] # [doc = " ```no_run"] # [doc = " let _ = (0..4).find_map(|x| Some(x + 1));"] # [doc = ""] # [doc = " // As there is no conditional check on the argument this could be written as:"] # [doc = " let _ = (0..4).map(|x| x + 1).next();"] # [doc = " ```"] # [clippy :: version = "1.61.0"] pub UNNECESSARY_FIND_MAP , complexity , "using `find_map` when a more succinct alternative exists" }
};
}
