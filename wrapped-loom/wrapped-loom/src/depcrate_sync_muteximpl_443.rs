// Generated macro for impl_443 (impl)
macro_rules! Depcrate_sync_muteximpl_443 {
() => {
// Module: crate::sync::mutex
// Provides: {"impl_443"}
// Dependencies: {}
impl < T > From < T > for Mutex < T > { # [doc = " Creates a new mutex in an unlocked state ready for use."] # [doc = " This is equivalent to [`Mutex::new`]."] fn from (t : T) -> Self { Self :: new (t) } }
};
}
