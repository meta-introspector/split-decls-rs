// Generated macro for impl_2601 (impl)
macro_rules! Depcrate_lock_bilockimpl_2601 {
() => {
// Module: crate::lock::bilock
// Provides: {"impl_2601"}
// Dependencies: {}
impl < T : Unpin > DerefMut for BiLockGuard < '_ , T > { fn deref_mut (& mut self) -> & mut T { unsafe { & mut * self . bilock . arc . value . as_ref () . unwrap () . get () } } }
};
}
