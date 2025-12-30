// Generated macro for impl_446 (impl)
macro_rules! Depcrate_sync_muteximpl_446 {
() => {
// Module: crate::sync::mutex
// Provides: {"impl_446"}
// Dependencies: {}
impl < 'a , T : ? Sized > ops :: DerefMut for MutexGuard < 'a , T > { fn deref_mut (& mut self) -> & mut T { self . data . as_mut () . unwrap () . deref_mut () } }
};
}
