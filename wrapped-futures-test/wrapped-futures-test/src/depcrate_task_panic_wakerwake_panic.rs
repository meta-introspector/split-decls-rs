// Generated macro for wake_panic (function)
macro_rules! Depcrate_task_panic_wakerwake_panic {
() => {
// Module: crate::task::panic_waker
// Provides: {"wake_panic"}
// Dependencies: {}
unsafe fn wake_panic (_data : * const ()) { if ! std :: thread :: panicking () { panic ! ("should not be woken") ; } }
};
}
