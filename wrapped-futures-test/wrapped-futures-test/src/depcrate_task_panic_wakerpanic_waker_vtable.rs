// Generated macro for PANIC_WAKER_VTABLE (const)
macro_rules! Depcrate_task_panic_wakerPANIC_WAKER_VTABLE {
() => {
// Module: crate::task::panic_waker
// Provides: {"PANIC_WAKER_VTABLE"}
// Dependencies: {}
const PANIC_WAKER_VTABLE : RawWakerVTable = RawWakerVTable :: new (clone_panic_waker , wake_panic , wake_panic , noop) ;
};
}
