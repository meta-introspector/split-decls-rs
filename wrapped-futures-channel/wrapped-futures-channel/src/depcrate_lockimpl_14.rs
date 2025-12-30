// Generated macro for impl_14 (impl)
macro_rules! Depcrate_lockimpl_14 {
() => {
// Module: crate::lock
// Provides: {"impl_14"}
// Dependencies: {}
impl < T > DerefMut for TryLock < '_ , T > { fn deref_mut (& mut self) -> & mut T { unsafe { & mut * self . __ptr . data . get () } } }
};
}
