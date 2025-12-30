// Generated macro for impl_2600 (impl)
macro_rules! Depcrate_lock_bilockimpl_2600 {
() => {
// Module: crate::lock::bilock
// Provides: {"impl_2600"}
// Dependencies: {}
impl < T > Deref for BiLockGuard < '_ , T > { type Target = T ; fn deref (& self) -> & T { unsafe { & * self . bilock . arc . value . as_ref () . unwrap () . get () } } }
};
}
