// Generated macro for impl_444 (impl)
macro_rules! Depcrate_sync_muteximpl_444 {
() => {
// Module: crate::sync::mutex
// Provides: {"impl_444"}
// Dependencies: {}
impl < 'a , T : ? Sized + 'a > MutexGuard < 'a , T > { pub (super) fn unborrow (& mut self) { self . data = None ; } pub (super) fn reborrow (& mut self) { self . data = Some (self . lock . data . lock () . unwrap ()) ; } pub (super) fn rt (& self) -> & rt :: Mutex { & self . lock . object } }
};
}
