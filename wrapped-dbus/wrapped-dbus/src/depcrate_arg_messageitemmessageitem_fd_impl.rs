// Generated macro for messageitem_fd_impl (module)
macro_rules! Depcrate_arg_messageitemmessageitem_fd_impl {
() => {
// Module: crate::arg::messageitem
// Provides: {"messageitem_fd_impl"}
// Dependencies: {}
# [cfg (feature = "stdfd")] mod messageitem_fd_impl { use super :: * ; impl Clone for MessageItemFd { fn clone (& self) -> Self { MessageItemFd (self . 0 . try_clone () . unwrap ()) } } impl PartialEq for MessageItemFd { fn eq (& self , _rhs : & Self) -> bool { false } } impl PartialOrd for MessageItemFd { fn partial_cmp (& self , other : & Self) -> Option < std :: cmp :: Ordering > { use std :: os :: unix :: io :: AsRawFd ; let a = self . 0 . as_raw_fd () ; let b = other . 0 . as_raw_fd () ; a . partial_cmp (& b) } } impl From < OwnedFd > for MessageItem { fn from (i : OwnedFd) -> MessageItem { MessageItem :: UnixFd (MessageItemFd (i)) } } impl < 'a > TryFrom < & 'a MessageItem > for & 'a OwnedFd { type Error = () ; fn try_from (i : & 'a MessageItem) -> Result < & 'a OwnedFd , () > { if let MessageItem :: UnixFd (ref b) = i { Ok (& b . 0) } else { Err (()) } } } }
};
}
