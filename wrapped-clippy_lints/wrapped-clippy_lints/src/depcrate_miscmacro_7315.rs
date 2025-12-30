// Generated macro for macro_7315 (macro)
macro_rules! Depcrate_miscmacro_7315 {
() => {
// Module: crate::misc
// Provides: {"macro_7315"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for the use of item with a single leading"] # [doc = " underscore."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " A single leading underscore is usually used to indicate"] # [doc = " that a item will not be used. Using such a item breaks this"] # [doc = " expectation."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn _foo() {}"] # [doc = ""] # [doc = " struct _FooStruct {}"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     _foo();"] # [doc = "     let _ = _FooStruct{};"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " fn foo() {}"] # [doc = ""] # [doc = " struct FooStruct {}"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     foo();"] # [doc = "     let _ = FooStruct{};"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.83.0"] pub USED_UNDERSCORE_ITEMS , pedantic , "using a item which is prefixed with an underscore" }
};
}
