// Generated macro for impl_485 (impl)
macro_rules! Depcrate_arg_messageitemimpl_485 {
() => {
// Module: crate::arg::messageitem
// Provides: {"impl_485"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a MessageItem > for & 'a Box < MessageItem > { type Error = () ; fn try_from (i : & 'a MessageItem) -> Result < & 'a Box < MessageItem > , () > { if let MessageItem :: Variant (b) = i { Ok (b) } else { Err (()) } } }
};
}
