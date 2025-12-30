// Generated macro for macro_11119 (macro)
macro_rules! Depcrate_zero_sized_map_valuesmacro_11119 {
() => {
// Module: crate::zero_sized_map_values
// Provides: {"macro_11119"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for maps with zero-sized value types anywhere in the code."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Since there is only a single value for a zero-sized type, a map"] # [doc = " containing zero sized values is effectively a set. Using a set in that case improves"] # [doc = " readability and communicates intent more clearly."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " * A zero-sized type cannot be recovered later if it contains private fields."] # [doc = " * This lints the signature of public items"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # use std::collections::HashMap;"] # [doc = " fn unique_words(text: &str) -> HashMap<&str, ()> {"] # [doc = "     todo!();"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # use std::collections::HashSet;"] # [doc = " fn unique_words(text: &str) -> HashSet<&str> {"] # [doc = "     todo!();"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.50.0"] pub ZERO_SIZED_MAP_VALUES , pedantic , "usage of map with zero-sized value type" }
};
}
