// Generated macro for impl_96 (impl)
macro_rules! Depcrate_collectorimpl_96 {
() => {
// Module: crate::collector
// Provides: {"impl_96"}
// Dependencies: {}
impl LocalHandle { # [doc = " Pins the handle."] # [inline] pub fn pin (& self) -> Guard { unsafe { (* self . local) . pin () } } # [doc = " Returns `true` if the handle is pinned."] # [inline] pub fn is_pinned (& self) -> bool { unsafe { (* self . local) . is_pinned () } } # [doc = " Returns the `Collector` associated with this handle."] # [inline] pub fn collector (& self) -> & Collector { unsafe { (* self . local) . collector () } } }
};
}
