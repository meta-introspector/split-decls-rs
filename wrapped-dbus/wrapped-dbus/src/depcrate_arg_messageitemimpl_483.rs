// Generated macro for impl_483 (impl)
macro_rules! Depcrate_arg_messageitemimpl_483 {
() => {
// Module: crate::arg::messageitem
// Provides: {"impl_483"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a MessageItem > for & 'a Path < 'static > { type Error = () ; fn try_from (i : & 'a MessageItem) -> Result < & 'a Path < 'static > , () > { if let MessageItem :: ObjectPath (b) = i . peel () { Ok (b) } else { Err (()) } } }
};
}
