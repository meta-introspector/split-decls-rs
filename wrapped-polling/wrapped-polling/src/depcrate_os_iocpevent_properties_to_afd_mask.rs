// Generated macro for event_properties_to_afd_mask (function)
macro_rules! Depcrate_os_iocpevent_properties_to_afd_mask {
() => {
// Module: crate::os::iocp
// Provides: {"event_properties_to_afd_mask"}
// Dependencies: {}
# [doc = " Translate an event to the mask expected by AFD."] # [inline] fn event_properties_to_afd_mask (readable : bool , writable : bool , error : bool) -> afd :: AfdPollMask { use afd :: AfdPollMask as AfdPoll ; let mut mask = AfdPoll :: empty () ; if error || readable || writable { mask |= AfdPoll :: ABORT | AfdPoll :: CONNECT_FAIL ; } if readable { mask |= AfdPoll :: RECEIVE | AfdPoll :: ACCEPT | AfdPoll :: DISCONNECT | AfdPoll :: RECEIVE_EXPEDITED ; } if writable { mask |= AfdPoll :: SEND ; } mask }
};
}
