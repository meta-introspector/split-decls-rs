// Generated macro for impl_250 (impl)
macro_rules! Depcrate_workloopimpl_250 {
() => {
// Module: crate::workloop
// Provides: {"impl_250"}
// Dependencies: {}
impl Deref for DispatchWorkloop { type Target = DispatchQueue ; # [doc = " Access the workloop as a [`DispatchQueue`]."] # [inline] fn deref (& self) -> & Self :: Target { let ptr : * const DispatchWorkloop = self ; let ptr : * const DispatchQueue = ptr . cast () ; unsafe { & * ptr } } }
};
}
