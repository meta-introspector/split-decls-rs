// Generated macro for macro_10619 (macro)
macro_rules! Depcrate_typesmacro_10619 {
() => {
// Module: crate::types
// Provides: {"macro_10619"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `Rc<Mutex<T>>`."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " `Rc` is used in single thread and `Mutex` is used in multi thread."] # [doc = " Consider using `Rc<RefCell<T>>` in single thread or `Arc<Mutex<T>>` in multi thread."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " Sometimes combining generic types can lead to the requirement that a"] # [doc = " type use Rc in conjunction with Mutex. We must consider those cases false positives, but"] # [doc = " alas they are quite hard to rule out. Luckily they are also rare."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " use std::rc::Rc;"] # [doc = " use std::sync::Mutex;"] # [doc = " fn foo(interned: Rc<Mutex<i32>>) { ... }"] # [doc = " ```"] # [doc = ""] # [doc = " Better:"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " use std::rc::Rc;"] # [doc = " use std::cell::RefCell"] # [doc = " fn foo(interned: Rc<RefCell<i32>>) { ... }"] # [doc = " ```"] # [clippy :: version = "1.55.0"] pub RC_MUTEX , restriction , "usage of `Rc<Mutex<T>>`" }
};
}
