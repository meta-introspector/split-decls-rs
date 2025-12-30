// Generated macro for impl_13 (impl)
macro_rules! Depcrate_lockimpl_13 {
() => {
// Module: crate::lock
// Provides: {"impl_13"}
// Dependencies: {}
impl < 'a , T > DerefMut for TryLock < 'a , T > { fn deref_mut (& mut self) -> & mut T { unsafe { & mut * self . __ptr . data . get () } } }
};
}
