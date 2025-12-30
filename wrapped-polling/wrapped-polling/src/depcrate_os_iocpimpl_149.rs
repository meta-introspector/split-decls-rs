// Generated macro for impl_149 (impl)
macro_rules! Depcrate_os_iocpimpl_149 {
() => {
// Module: crate::os::iocp
// Provides: {"impl_149"}
// Dependencies: {}
impl EventExtra { # [doc = " Create a new, empty version of this struct."] # [inline] pub const fn empty () -> EventExtra { EventExtra { flags : AfdPollMask :: empty () , } } # [doc = " Is this a HUP event?"] # [inline] pub fn is_hup (& self) -> bool { self . flags . intersects (AfdPollMask :: ABORT) } # [doc = " Is this a PRI event?"] # [inline] pub fn is_pri (& self) -> bool { self . flags . intersects (AfdPollMask :: RECEIVE_EXPEDITED) } # [doc = " Set up a listener for HUP events."] # [inline] pub fn set_hup (& mut self , active : bool) { self . flags . set (AfdPollMask :: ABORT , active) ; } # [doc = " Set up a listener for PRI events."] # [inline] pub fn set_pri (& mut self , active : bool) { self . flags . set (AfdPollMask :: RECEIVE_EXPEDITED , active) ; } # [doc = " Check if TCP connect failed. Deprecated."] # [inline] pub fn is_connect_failed (& self) -> Option < bool > { Some (self . flags . intersects (AfdPollMask :: CONNECT_FAIL)) } # [doc = " Check if TCP connect failed."] # [inline] pub fn is_err (& self) -> Option < bool > { Some (self . flags . intersects (AfdPollMask :: CONNECT_FAIL)) } }
};
}
