// Generated macro for macro_9427 (macro)
macro_rules! Depcrate_set_contains_or_insertmacro_9427 {
() => {
// Module: crate::set_contains_or_insert
// Provides: {"macro_9427"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `contains` to see if a value is not present"] # [doc = " in a set like `HashSet` or `BTreeSet`, followed by an `insert`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Using just `insert` and checking the returned `bool` is more efficient."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " In case the value that wants to be inserted is borrowed and also expensive or impossible"] # [doc = " to clone. In such a scenario, the developer might want to check with `contains` before inserting,"] # [doc = " to avoid the clone. In this case, it will report a false positive."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust"] # [doc = " use std::collections::HashSet;"] # [doc = " let mut set = HashSet::new();"] # [doc = " let value = 5;"] # [doc = " if !set.contains(&value) {"] # [doc = "     set.insert(value);"] # [doc = "     println!(\"inserted {value:?}\");"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```rust"] # [doc = " use std::collections::HashSet;"] # [doc = " let mut set = HashSet::new();"] # [doc = " let value = 5;"] # [doc = " if set.insert(&value) {"] # [doc = "     println!(\"inserted {value:?}\");"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.81.0"] pub SET_CONTAINS_OR_INSERT , nursery , "call to `<set>::contains` followed by `<set>::insert`" }
};
}
