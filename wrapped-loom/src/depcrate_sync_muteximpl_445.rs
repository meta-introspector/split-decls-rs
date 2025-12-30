// Generated macro for impl_445 (impl)
macro_rules! Depcrate_sync_muteximpl_445 {
() => {
// Module: crate::sync::mutex
// Provides: {"impl_445"}
// Dependencies: {}
impl < 'a , T : ? Sized > ops :: Deref for MutexGuard < 'a , T > { type Target = T ; fn deref (& self) -> & T { self . data . as_ref () . unwrap () . deref () } }
};
}
