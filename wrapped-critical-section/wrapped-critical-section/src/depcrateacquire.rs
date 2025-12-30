// Generated macro for acquire (function)
macro_rules! Depcrateacquire {
() => {
// Module: crate
// Provides: {"acquire"}
// Dependencies: {}
# [doc = " Acquire a critical section in the current thread."] # [doc = ""] # [doc = " This function is extremely low level. Strongly prefer using [`with`] instead."] # [doc = ""] # [doc = " Nesting critical sections is allowed. The inner critical sections"] # [doc = " are mostly no-ops since they're already protected by the outer one."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - Each `acquire` call must be paired with exactly one `release` call in the same thread."] # [doc = " - `acquire` returns a \"restore state\" that you must pass to the corresponding `release` call."] # [doc = " - `acquire`/`release` pairs must be \"properly nested\", ie it's not OK to do `a=acquire(); b=acquire(); release(a); release(b);`."] # [doc = " - It is UB to call `release` if the critical section is not acquired in the current thread."] # [doc = " - It is UB to call `release` with a \"restore state\" that does not come from the corresponding `acquire` call."] # [doc = " - It must provide ordering guarantees at least equivalent to a [`core::sync::atomic::Ordering::Acquire`]"] # [doc = "   on a memory location shared by all critical sections, on which the `release` call will do a"] # [doc = "   [`core::sync::atomic::Ordering::Release`] operation."] # [inline (always)] pub unsafe fn acquire () -> RestoreState { extern "Rust" { fn _critical_section_1_0_acquire () -> RawRestoreState ; } # [allow (clippy :: unit_arg)] RestoreState (_critical_section_1_0_acquire ()) }
};
}
