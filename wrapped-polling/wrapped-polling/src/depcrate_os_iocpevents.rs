// Generated macro for Events (struct)
macro_rules! Depcrate_os_iocpEvents {
() => {
// Module: crate::os::iocp
// Provides: {"Events"}
// Dependencies: {}
# [doc = " The container for events."] pub (super) struct Events { # [doc = " List of IOCP packets."] packets : Vec < Event > , # [doc = " Buffer for completion packets."] completions : Vec < OverlappedEntry < Packet > > , }
};
}
