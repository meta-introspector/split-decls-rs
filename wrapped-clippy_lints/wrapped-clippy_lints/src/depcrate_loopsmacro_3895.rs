// Generated macro for macro_3895 (macro)
macro_rules! Depcrate_loopsmacro_3895 {
() => {
// Module: crate::loops
// Provides: {"macro_3895"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for iterating a map (`HashMap` or `BTreeMap`) and"] # [doc = " ignoring either the keys or values."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Readability. There are `keys` and `values` methods that"] # [doc = " can be used to express that don't need the values or keys."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```ignore"] # [doc = " for (k, _) in &map {"] # [doc = "     .."] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " could be replaced by"] # [doc = ""] # [doc = " ```ignore"] # [doc = " for k in map.keys() {"] # [doc = "     .."] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub FOR_KV_MAP , style , "looping on a map using `iter` when `keys` or `values` would do" }
};
}
