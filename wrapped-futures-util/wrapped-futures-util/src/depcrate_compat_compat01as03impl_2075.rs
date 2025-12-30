// Generated macro for impl_2075 (impl)
macro_rules! Depcrate_compat_compat01as03impl_2075 {
() => {
// Module: crate::compat::compat01as03
// Provides: {"impl_2075"}
// Dependencies: {}
unsafe impl UnsafeNotify01 for NotifyWaker { unsafe fn clone_raw (& self) -> NotifyHandle01 { WakerToHandle (& self . 0) . into () } unsafe fn drop_raw (& self) { let ptr : * const dyn UnsafeNotify01 = self ; drop (unsafe { Box :: from_raw (ptr as * mut dyn UnsafeNotify01) }) ; } }
};
}
