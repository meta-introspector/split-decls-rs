// Generated macro for impl_13 (impl)
macro_rules! Depcrate_lockimpl_13 {
() => {
// Module: crate::lock
// Provides: {"impl_13"}
// Dependencies: {}
impl < T > Deref for TryLock < '_ , T > { type Target = T ; fn deref (& self) -> & T { unsafe { & * self . __ptr . data . get () } } }
};
}
