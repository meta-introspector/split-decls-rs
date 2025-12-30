// Generated macro for macro_1830 (macro)
macro_rules! Depcrate_empty_dropmacro_1830 {
() => {
// Module: crate::empty_drop
// Provides: {"macro_1830"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for empty `Drop` implementations."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Empty `Drop` implementations have no effect when dropping an instance of the type. They are"] # [doc = " most likely useless. However, an empty `Drop` implementation prevents a type from being"] # [doc = " destructured, which might be the intention behind adding the implementation as a marker."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " struct S;"] # [doc = ""] # [doc = " impl Drop for S {"] # [doc = "     fn drop(&mut self) {}"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " struct S;"] # [doc = " ```"] # [clippy :: version = "1.62.0"] pub EMPTY_DROP , restriction , "empty `Drop` implementations" }
};
}
