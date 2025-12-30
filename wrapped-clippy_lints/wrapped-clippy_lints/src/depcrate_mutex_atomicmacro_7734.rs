// Generated macro for macro_7734 (macro)
macro_rules! Depcrate_mutex_atomicmacro_7734 {
() => {
// Module: crate::mutex_atomic
// Provides: {"macro_7734"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `Mutex<X>` where an atomic will do."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Using a mutex just to make access to a plain bool or"] # [doc = " reference sequential is shooting flies with cannons."] # [doc = " `std::sync::atomic::AtomicBool` and `std::sync::atomic::AtomicPtr` are leaner and"] # [doc = " faster."] # [doc = ""] # [doc = " On the other hand, `Mutex`es are, in general, easier to"] # [doc = " verify correctness. An atomic does not behave the same as"] # [doc = " an equivalent mutex. See [this issue](https://github.com/rust-lang/rust-clippy/issues/4295)'s"] # [doc = " commentary for more details."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " * This lint cannot detect if the mutex is actually used"] # [doc = "   for waiting before a critical section."] # [doc = " * This lint has a false positive that warns without considering the case"] # [doc = "   where `Mutex` is used together with `Condvar`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let y = true;"] # [doc = " # use std::sync::Mutex;"] # [doc = " let x = Mutex::new(&y);"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let y = true;"] # [doc = " # use std::sync::atomic::AtomicBool;"] # [doc = " let x = AtomicBool::new(y);"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub MUTEX_ATOMIC , restriction , "using a mutex where an atomic value could be used instead." }
};
}
