// Generated macro for impl_409 (impl)
macro_rules! Depcrate_future_future_groupimpl_409 {
() => {
// Module: crate::future::future_group
// Provides: {"impl_409"}
// Dependencies: {}
impl < F : Future > Deref for Keyed < F > { type Target = FutureGroup < F > ; fn deref (& self) -> & Self :: Target { & self . group } }
};
}
