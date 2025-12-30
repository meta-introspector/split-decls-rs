// Generated macro for macro_1792 (macro)
macro_rules! Depcrate_drop_forget_refmacro_1792 {
() => {
// Module: crate::drop_forget_ref
// Provides: {"macro_1792"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for calls to `std::mem::drop` with a value that does not implement `Drop`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Calling `std::mem::drop` is no different than dropping such a type. A different value may"] # [doc = " have been intended."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " struct Foo;"] # [doc = " let x = Foo;"] # [doc = " std::mem::drop(x);"] # [doc = " ```"] # [clippy :: version = "1.62.0"] pub DROP_NON_DROP , suspicious , "call to `std::mem::drop` with a value which does not implement `Drop`" }
};
}
