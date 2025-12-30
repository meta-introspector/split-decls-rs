// Generated macro for impl_29 (impl)
macro_rules! Depcrate_muteximpl_29 {
() => {
// Module: crate::mutex
// Provides: {"impl_29"}
// Dependencies: {}
impl < R : RawMutex , T > From < T > for Mutex < R , T > { # [inline] fn from (t : T) -> Mutex < R , T > { Mutex :: new (t) } }
};
}
