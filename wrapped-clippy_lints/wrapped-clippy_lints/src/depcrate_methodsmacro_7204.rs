// Generated macro for macro_7204 (macro)
macro_rules! Depcrate_methodsmacro_7204 {
() => {
// Module: crate::methods
// Provides: {"macro_7204"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = ""] # [doc = " Checks for iterating a map (`HashMap` or `BTreeMap`) and"] # [doc = " ignoring either the keys or values."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = ""] # [doc = " Readability. There are `keys` and `values` methods that"] # [doc = " can be used to express that we only need the keys or the values."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # use std::collections::HashMap;"] # [doc = " let map: HashMap<u32, u32> = HashMap::new();"] # [doc = " let values = map.iter().map(|(_, value)| value).collect::<Vec<_>>();"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # use std::collections::HashMap;"] # [doc = " let map: HashMap<u32, u32> = HashMap::new();"] # [doc = " let values = map.values().collect::<Vec<_>>();"] # [doc = " ```"] # [clippy :: version = "1.66.0"] pub ITER_KV_MAP , complexity , "iterating on map using `iter` when `keys` or `values` would do" }
};
}
