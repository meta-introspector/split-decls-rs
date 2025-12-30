// Generated macro for macro_10733 (macro)
macro_rules! Depcrate_unused_peekablemacro_10733 {
() => {
// Module: crate::unused_peekable
// Provides: {"macro_10733"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for the creation of a `peekable` iterator that is never `.peek()`ed"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Creating a peekable iterator without using any of its methods is likely a mistake,"] # [doc = " or just a leftover after a refactor."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let collection = vec![1, 2, 3];"] # [doc = " let iter = collection.iter().peekable();"] # [doc = ""] # [doc = " for item in iter {"] # [doc = "     // ..."] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let collection = vec![1, 2, 3];"] # [doc = " let iter = collection.iter();"] # [doc = ""] # [doc = " for item in iter {"] # [doc = "     // ..."] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.65.0"] pub UNUSED_PEEKABLE , nursery , "creating a peekable iterator without using any of its methods" }
};
}
