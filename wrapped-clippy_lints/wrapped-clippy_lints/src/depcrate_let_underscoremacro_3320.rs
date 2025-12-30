// Generated macro for macro_3320 (macro)
macro_rules! Depcrate_let_underscoremacro_3320 {
() => {
// Module: crate::let_underscore
// Provides: {"macro_3320"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `let _ = sync_lock`. This supports `mutex` and `rwlock` in"] # [doc = " `parking_lot`. For `std` locks see the `rustc` lint"] # [doc = " [`let_underscore_lock`](https://doc.rust-lang.org/nightly/rustc/lints/listing/deny-by-default.html#let-underscore-lock)"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This statement immediately drops the lock instead of"] # [doc = " extending its lifetime to the end of the scope, which is often not intended."] # [doc = " To extend lock lifetime to the end of the scope, use an underscore-prefixed"] # [doc = " name instead (i.e. _lock). If you want to explicitly drop the lock,"] # [doc = " `std::mem::drop` conveys your intention better and is less error-prone."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " let _ = mutex.lock();"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " let _lock = mutex.lock();"] # [doc = " ```"] # [clippy :: version = "1.43.0"] pub LET_UNDERSCORE_LOCK , correctness , "non-binding `let` on a synchronization lock" }
};
}
