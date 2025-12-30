// Generated macro for impl_28 (impl)
macro_rules! Depcrate_muteximpl_28 {
() => {
// Module: crate::mutex
// Provides: {"impl_28"}
// Dependencies: {}
impl < R : RawMutex , T : ? Sized + Default > Default for Mutex < R , T > { # [inline] fn default () -> Mutex < R , T > { Mutex :: new (Default :: default ()) } }
};
}
