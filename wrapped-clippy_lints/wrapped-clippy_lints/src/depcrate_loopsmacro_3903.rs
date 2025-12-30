// Generated macro for macro_3903 (macro)
macro_rules! Depcrate_loopsmacro_3903 {
() => {
// Module: crate::loops
// Provides: {"macro_3903"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for empty spin loops"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The loop body should have something like `thread::park()` or at least"] # [doc = " `std::hint::spin_loop()` to avoid needlessly burning cycles and conserve"] # [doc = " energy. Perhaps even better use an actual lock, if possible."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " This lint doesn't currently trigger on `while let` or"] # [doc = " `loop { match .. { .. } }` loops, which would be considered idiomatic in"] # [doc = " combination with e.g. `AtomicBool::compare_exchange_weak`."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```ignore"] # [doc = " use core::sync::atomic::{AtomicBool, Ordering};"] # [doc = " let b = AtomicBool::new(true);"] # [doc = " // give a ref to `b` to another thread,wait for it to become false"] # [doc = " while b.load(Ordering::Acquire) {};"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```rust,no_run"] # [doc = "# use core::sync::atomic::{AtomicBool, Ordering};"] # [doc = "# let b = AtomicBool::new(true);"] # [doc = " while b.load(Ordering::Acquire) {"] # [doc = "     std::hint::spin_loop()"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.61.0"] pub MISSING_SPIN_LOOP , perf , "An empty busy waiting loop" }
};
}
