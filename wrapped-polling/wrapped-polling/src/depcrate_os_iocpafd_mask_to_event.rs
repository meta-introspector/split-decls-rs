// Generated macro for afd_mask_to_event (function)
macro_rules! Depcrate_os_iocpafd_mask_to_event {
() => {
// Module: crate::os::iocp
// Provides: {"afd_mask_to_event"}
// Dependencies: {}
# [doc = " Convert the mask reported by AFD to an event."] # [inline] fn afd_mask_to_event (mask : afd :: AfdPollMask) -> (bool , bool) { use afd :: AfdPollMask as AfdPoll ; let mut readable = false ; let mut writable = false ; if mask . intersects (AfdPoll :: RECEIVE | AfdPoll :: ACCEPT | AfdPoll :: DISCONNECT | AfdPoll :: RECEIVE_EXPEDITED ,) { readable = true ; } if mask . intersects (AfdPoll :: SEND) { writable = true ; } if mask . intersects (AfdPoll :: ABORT | AfdPoll :: CONNECT_FAIL) { readable = true ; writable = true ; } (readable , writable) }
};
}
