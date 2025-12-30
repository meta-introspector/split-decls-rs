// Generated macro for impl_129 (impl)
macro_rules! Depcrate_waiterimpl_129 {
() => {
// Module: crate::waiter
// Provides: {"impl_129"}
// Dependencies: {}
impl WaiterSignaler { # [doc = " # Safety"] # [doc = " Signals the `Waiter`. This is unsafe because the lifetime of `WaiterSignaler` is not tied"] # [doc = " to the lifetime of the `Waiter`. This is not possible in this case because the `Waiter`"] # [doc = " is used to signal a WinRT async completion and the compiler doesn't know that the lifetime"] # [doc = " of the delegate is bounded by the calling function."] pub unsafe fn signal (& self) { unsafe { SetEvent (self . 0) ; } } }
};
}
