// Generated macro for macro_10670 (macro)
macro_rules! Depcrate_unsafe_removed_from_namemacro_10670 {
() => {
// Module: crate::unsafe_removed_from_name
// Provides: {"macro_10670"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for imports that remove \"unsafe\" from an item's"] # [doc = " name."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Renaming makes it less clear which traits and"] # [doc = " structures are unsafe."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " use std::cell::{UnsafeCell as TotallySafeCell};"] # [doc = ""] # [doc = " extern crate crossbeam;"] # [doc = " use crossbeam::{spawn_unsafe as spawn};"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub UNSAFE_REMOVED_FROM_NAME , style , "`unsafe` removed from API names on import" }
};
}
