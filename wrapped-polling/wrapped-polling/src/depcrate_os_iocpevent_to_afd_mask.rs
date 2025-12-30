// Generated macro for event_to_afd_mask (function)
macro_rules! Depcrate_os_iocpevent_to_afd_mask {
() => {
// Module: crate::os::iocp
// Provides: {"event_to_afd_mask"}
// Dependencies: {}
# [doc = " Translate an event to the mask expected by AFD."] # [inline] fn event_to_afd_mask (event : Event , error : bool) -> afd :: AfdPollMask { event_properties_to_afd_mask (event . readable , event . writable , error) | event . extra . flags }
};
}
