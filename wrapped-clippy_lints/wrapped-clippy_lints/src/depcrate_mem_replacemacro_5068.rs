// Generated macro for macro_5068 (macro)
macro_rules! Depcrate_mem_replacemacro_5068 {
() => {
// Module: crate::mem_replace
// Provides: {"macro_5068"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `std::mem::replace` on a value of type"] # [doc = " `T` with `T::default()`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `std::mem` module already has the method `take` to"] # [doc = " take the current value and replace it with the default value of that type."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let mut text = String::from(\"foo\");"] # [doc = " let replaced = std::mem::replace(&mut text, String::default());"] # [doc = " ```"] # [doc = " Is better expressed with:"] # [doc = " ```no_run"] # [doc = " let mut text = String::from(\"foo\");"] # [doc = " let taken = std::mem::take(&mut text);"] # [doc = " ```"] # [clippy :: version = "1.42.0"] pub MEM_REPLACE_WITH_DEFAULT , style , "replacing a value of type `T` with `T::default()` instead of using `std::mem::take`" }
};
}
