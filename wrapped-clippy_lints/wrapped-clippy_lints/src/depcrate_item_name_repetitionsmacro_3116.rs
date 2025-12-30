// Generated macro for macro_3116 (macro)
macro_rules! Depcrate_item_name_repetitionsmacro_3116 {
() => {
// Module: crate::item_name_repetitions
// Provides: {"macro_3116"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Detects public item names that are prefixed or suffixed by the"] # [doc = " containing public module's name."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It requires the user to type the module name twice in each usage,"] # [doc = " especially if they choose to import the module rather than its contents."] # [doc = ""] # [doc = " Lack of such repetition is also the style used in the Rust standard library;"] # [doc = " e.g. `io::Error` and `fmt::Error` rather than `io::IoError` and `fmt::FmtError`;"] # [doc = " and `array::from_ref` rather than `array::array_from_ref`."] # [doc = ""] # [doc = " ### Known issues"] # [doc = " Glob re-exports are ignored; e.g. this will not warn even though it should:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " pub mod foo {"] # [doc = "     mod iteration {"] # [doc = "         pub struct FooIter {}"] # [doc = "     }"] # [doc = "     pub use iteration::*; // creates the path `foo::FooIter`"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " mod cake {"] # [doc = "     struct BlackForestCake;"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " mod cake {"] # [doc = "     struct BlackForest;"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.33.0"] pub MODULE_NAME_REPETITIONS , restriction , "type names prefixed/postfixed with their containing module's name" }
};
}
