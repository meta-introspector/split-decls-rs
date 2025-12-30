// Generated macro for macro_10791 (macro)
macro_rules! Depcrate_unused_trait_namesmacro_10791 {
() => {
// Module: crate::unused_trait_names
// Provides: {"macro_10791"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `use Trait` where the Trait is only used for its methods and not referenced by a path directly."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Traits imported that aren't used directly can be imported anonymously with `use Trait as _`."] # [doc = " It is more explicit, avoids polluting the current scope with unused names and can be useful to show which imports are required for traits."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " use std::fmt::Write;"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     let mut s = String::new();"] # [doc = "     let _ = write!(s, \"hello, world!\");"] # [doc = "     println!(\"{s}\");"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " use std::fmt::Write as _;"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     let mut s = String::new();"] # [doc = "     let _ = write!(s, \"hello, world!\");"] # [doc = "     println!(\"{s}\");"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.83.0"] pub UNUSED_TRAIT_NAMES , restriction , "use items that import a trait but only use it anonymously" }
};
}
