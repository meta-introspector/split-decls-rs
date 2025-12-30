// Generated macro for macro_7218 (macro)
macro_rules! Depcrate_methodsmacro_7218 {
() => {
// Module: crate::methods
// Provides: {"macro_7218"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Looks for calls to `RwLock::write` where the lock is only used for reading."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The write portion of `RwLock` is exclusive, meaning that no other thread"] # [doc = " can access the lock while this writer is active."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " use std::sync::RwLock;"] # [doc = " fn assert_is_zero(lock: &RwLock<i32>) {"] # [doc = "     let num = lock.write().unwrap();"] # [doc = "     assert_eq!(*num, 0);"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " use std::sync::RwLock;"] # [doc = " fn assert_is_zero(lock: &RwLock<i32>) {"] # [doc = "     let num = lock.read().unwrap();"] # [doc = "     assert_eq!(*num, 0);"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.73.0"] pub READONLY_WRITE_LOCK , perf , "acquiring a write lock when a read lock would work" }
};
}
