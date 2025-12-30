// Generated macro for impl_2073 (impl)
macro_rules! Depcrate_compat_compat01as03impl_2073 {
() => {
// Module: crate::compat::compat01as03
// Provides: {"impl_2073"}
// Dependencies: {}
impl From < WakerToHandle < '_ > > for NotifyHandle01 { fn from (handle : WakerToHandle < '_ >) -> Self { let ptr = Box :: new (NotifyWaker (handle . 0 . clone ())) ; unsafe { Self :: new (Box :: into_raw (ptr)) } } }
};
}
