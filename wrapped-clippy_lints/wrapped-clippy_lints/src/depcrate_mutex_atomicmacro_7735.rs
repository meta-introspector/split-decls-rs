// Generated macro for macro_7735 (macro)
macro_rules! Depcrate_mutex_atomicmacro_7735 {
() => {
// Module: crate::mutex_atomic
// Provides: {"macro_7735"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `Mutex<X>` where `X` is an integral"] # [doc = " type."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Using a mutex just to make access to a plain integer"] # [doc = " sequential is"] # [doc = " shooting flies with cannons. `std::sync::atomic::AtomicUsize` is leaner and faster."] # [doc = ""] # [doc = " On the other hand, `Mutex`es are, in general, easier to"] # [doc = " verify correctness. An atomic does not behave the same as"] # [doc = " an equivalent mutex. See [this issue](https://github.com/rust-lang/rust-clippy/issues/4295)'s"] # [doc = " commentary for more details."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " * This lint cannot detect if the mutex is actually used"] # [doc = "   for waiting before a critical section."] # [doc = " * This lint has a false positive that warns without considering the case"] # [doc = "   where `Mutex` is used together with `Condvar`."] # [doc = " * This lint suggest using `AtomicU64` instead of `Mutex<u64>`, but"] # [doc = "   `AtomicU64` is not available on some 32-bit platforms."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # use std::sync::Mutex;"] # [doc = " let x = Mutex::new(0usize);"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # use std::sync::atomic::AtomicUsize;"] # [doc = " let x = AtomicUsize::new(0usize);"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub MUTEX_INTEGER , restriction , "using a mutex for an integer type" }
};
}
