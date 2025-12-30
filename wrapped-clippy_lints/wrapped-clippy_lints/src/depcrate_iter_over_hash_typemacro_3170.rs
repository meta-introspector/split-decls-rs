// Generated macro for macro_3170 (macro)
macro_rules! Depcrate_iter_over_hash_typemacro_3170 {
() => {
// Module: crate::iter_over_hash_type
// Provides: {"macro_3170"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " This is a restriction lint which prevents the use of hash types (i.e., `HashSet` and `HashMap`) in for loops."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Because hash types are unordered, when iterated through such as in a `for` loop, the values are returned in"] # [doc = " an undefined order. As a result, on redundant systems this may cause inconsistencies and anomalies."] # [doc = " In addition, the unknown order of the elements may reduce readability or introduce other undesired"] # [doc = " side effects."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = "     let my_map = std::collections::HashMap::<i32, String>::new();"] # [doc = "     for (key, value) in my_map { /* ... */ }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = "     let my_map = std::collections::HashMap::<i32, String>::new();"] # [doc = "     let mut keys = my_map.keys().clone().collect::<Vec<_>>();"] # [doc = "     keys.sort();"] # [doc = "     for key in keys {"] # [doc = "         let value = &my_map[key];"] # [doc = "     }"] # [doc = " ```"] # [clippy :: version = "1.76.0"] pub ITER_OVER_HASH_TYPE , restriction , "iterating over unordered hash-based types (`HashMap` and `HashSet`)" }
};
}
