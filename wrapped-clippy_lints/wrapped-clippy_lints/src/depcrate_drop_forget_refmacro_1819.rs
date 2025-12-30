// Generated macro for macro_1819 (macro)
macro_rules! Depcrate_drop_forget_refmacro_1819 {
() => {
// Module: crate::drop_forget_ref
// Provides: {"macro_1819"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for calls to `std::mem::forget` with a value that does not implement `Drop`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Calling `std::mem::forget` is no different than dropping such a type. A different value may"] # [doc = " have been intended."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " struct Foo;"] # [doc = " let x = Foo;"] # [doc = " std::mem::forget(x);"] # [doc = " ```"] # [clippy :: version = "1.62.0"] pub FORGET_NON_DROP , suspicious , "call to `std::mem::forget` with a value which does not implement `Drop`" }
};
}
