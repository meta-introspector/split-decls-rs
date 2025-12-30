// Generated macro for macro_10616 (macro)
macro_rules! Depcrate_typesmacro_10616 {
() => {
// Module: crate::types
// Provides: {"macro_10616"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of redundant allocations anywhere in the code."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Expressions such as `Rc<&T>`, `Rc<Rc<T>>`, `Rc<Arc<T>>`, `Rc<Box<T>>`, `Arc<&T>`, `Arc<Rc<T>>`,"] # [doc = " `Arc<Arc<T>>`, `Arc<Box<T>>`, `Box<&T>`, `Box<Rc<T>>`, `Box<Arc<T>>`, `Box<Box<T>>`, add an unnecessary level of indirection."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # use std::rc::Rc;"] # [doc = " fn foo(bar: Rc<&usize>) {}"] # [doc = " ```"] # [doc = ""] # [doc = " Better:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " fn foo(bar: &usize) {}"] # [doc = " ```"] # [clippy :: version = "1.44.0"] pub REDUNDANT_ALLOCATION , perf , "redundant allocation" }
};
}
