// Generated macro for macro_2619 (macro)
macro_rules! Depcrate_if_let_mutexmacro_2619 {
() => {
// Module: crate::if_let_mutex
// Provides: {"macro_2619"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `Mutex::lock` calls in `if let` expression"] # [doc = " with lock calls in any of the else blocks."] # [doc = ""] # [doc = " ### Disabled starting in Edition 2024"] # [doc = " This lint is effectively disabled starting in"] # [doc = " Edition 2024 as `if let ... else` scoping was reworked"] # [doc = " such that this is no longer an issue. See"] # [doc = " [Proposal: stabilize if_let_rescope for Edition 2024](https://github.com/rust-lang/rust/issues/131154)"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The Mutex lock remains held for the whole"] # [doc = " `if let ... else` block and deadlocks."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " if let Ok(thing) = mutex.lock() {"] # [doc = "     do_thing();"] # [doc = " } else {"] # [doc = "     mutex.lock();"] # [doc = " }"] # [doc = " ```"] # [doc = " Should be written"] # [doc = " ```rust,ignore"] # [doc = " let locked = mutex.lock();"] # [doc = " if let Ok(thing) = locked {"] # [doc = "     do_thing(thing);"] # [doc = " } else {"] # [doc = "     use_locked(locked);"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.45.0"] pub IF_LET_MUTEX , correctness , "locking a `Mutex` in an `if let` block can cause deadlocks" }
};
}
