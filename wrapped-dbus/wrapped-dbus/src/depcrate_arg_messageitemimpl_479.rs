// Generated macro for impl_479 (impl)
macro_rules! Depcrate_arg_messageitemimpl_479 {
() => {
// Module: crate::arg::messageitem
// Provides: {"impl_479"}
// Dependencies: {}
# [cfg (unix)] impl From < std :: fs :: File > for MessageItem { fn from (i : std :: fs :: File) -> MessageItem { use std :: os :: unix :: io :: { FromRawFd , IntoRawFd } ; let fd = unsafe { OwnedFd :: from_raw_fd (i . into_raw_fd ()) } ; fd . into () } }
};
}
