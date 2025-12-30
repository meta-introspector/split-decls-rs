// Generated macro for macro_1929 (macro)
macro_rules! Depcrate_entrymacro_1929 {
() => {
// Module: crate::entry
// Provides: {"macro_1929"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `contains_key` + `insert` on `HashMap`"] # [doc = " or `BTreeMap`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Using `entry` is more efficient."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " The suggestion may have type inference errors in some cases. e.g."] # [doc = " ```no_run"] # [doc = " let mut map = std::collections::HashMap::new();"] # [doc = " let _ = if !map.contains_key(&0) {"] # [doc = "     map.insert(0, 0)"] # [doc = " } else {"] # [doc = "     None"] # [doc = " };"] # [doc = " ```"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # use std::collections::HashMap;"] # [doc = " # let mut map = HashMap::new();"] # [doc = " # let k = 1;"] # [doc = " # let v = 1;"] # [doc = " if !map.contains_key(&k) {"] # [doc = "     map.insert(k, v);"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # use std::collections::HashMap;"] # [doc = " # let mut map = HashMap::new();"] # [doc = " # let k = 1;"] # [doc = " # let v = 1;"] # [doc = " map.entry(k).or_insert(v);"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub MAP_ENTRY , perf , "use of `contains_key` followed by `insert` on a `HashMap` or `BTreeMap`" }
};
}
