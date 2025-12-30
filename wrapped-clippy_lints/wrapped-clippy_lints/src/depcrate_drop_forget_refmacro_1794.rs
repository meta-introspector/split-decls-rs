// Generated macro for macro_1794 (macro)
macro_rules! Depcrate_drop_forget_refmacro_1794 {
() => {
// Module: crate::drop_forget_ref
// Provides: {"macro_1794"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `std::mem::forget(t)` where `t` is"] # [doc = " `Drop` or has a field that implements `Drop`."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " `std::mem::forget(t)` prevents `t` from running its destructor, possibly causing leaks."] # [doc = " It is not possible to detect all means of creating leaks, but it may be desirable to"] # [doc = " prohibit the simple ones."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # use std::mem;"] # [doc = " # use std::rc::Rc;"] # [doc = " mem::forget(Rc::new(55))"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub MEM_FORGET , restriction , "`mem::forget` usage on `Drop` types, likely to cause memory leaks" }
};
}
