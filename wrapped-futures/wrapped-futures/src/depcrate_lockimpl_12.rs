// Generated macro for impl_12 (impl)
macro_rules! Depcrate_lockimpl_12 {
() => {
// Module: crate::lock
// Provides: {"impl_12"}
// Dependencies: {}
impl < 'a , T > Deref for TryLock < 'a , T > { type Target = T ; fn deref (& self) -> & T { unsafe { & * self . __ptr . data . get () } } }
};
}
