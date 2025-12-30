// Generated macro for macro_184 (macro)
macro_rules! Depcrate_arc_with_non_send_syncmacro_184 {
() => {
// Module: crate::arc_with_non_send_sync
// Provides: {"macro_184"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does."] # [doc = " This lint warns when you use `Arc` with a type that does not implement `Send` or `Sync`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `Arc<T>` is a thread-safe `Rc<T>` and guarantees that updates to the reference counter"] # [doc = " use atomic operations. To send an `Arc<T>` across thread boundaries and"] # [doc = " share ownership between multiple threads, `T` must be [both `Send` and `Sync`](https://doc.rust-lang.org/std/sync/struct.Arc.html#thread-safety),"] # [doc = " so either `T` should be made `Send + Sync` or an `Rc` should be used instead of an `Arc`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # use std::cell::RefCell;"] # [doc = " # use std::sync::Arc;"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     // This is fine, as `i32` implements `Send` and `Sync`."] # [doc = "     let a = Arc::new(42);"] # [doc = ""] # [doc = "     // `RefCell` is `!Sync`, so either the `Arc` should be replaced with an `Rc`"] # [doc = "     // or the `RefCell` replaced with something like a `RwLock`"] # [doc = "     let b = Arc::new(RefCell::new(42));"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.72.0"] pub ARC_WITH_NON_SEND_SYNC , suspicious , "using `Arc` with a type that does not implement `Send` and `Sync`" }
};
}
