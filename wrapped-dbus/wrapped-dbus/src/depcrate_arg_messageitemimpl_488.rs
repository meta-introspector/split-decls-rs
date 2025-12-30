// Generated macro for impl_488 (impl)
macro_rules! Depcrate_arg_messageitemimpl_488 {
() => {
// Module: crate::arg::messageitem
// Provides: {"impl_488"}
// Dependencies: {}
# [cfg (not (feature = "stdfd"))] impl < 'a > TryFrom < & 'a MessageItem > for & 'a OwnedFd { type Error = () ; fn try_from (i : & 'a MessageItem) -> Result < & 'a OwnedFd , () > { if let MessageItem :: UnixFd (ref b) = i { Ok (b) } else { Err (()) } } }
};
}
