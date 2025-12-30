// Generated macro for impl_486 (impl)
macro_rules! Depcrate_arg_messageitemimpl_486 {
() => {
// Module: crate::arg::messageitem
// Provides: {"impl_486"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a MessageItem > for & 'a Vec < MessageItem > { type Error = () ; fn try_from (i : & 'a MessageItem) -> Result < & 'a Vec < MessageItem > , () > { match i . peel () { MessageItem :: Array (b) => Ok (& b . v) , MessageItem :: Struct (b) => Ok (b) , _ => Err (()) , } } }
};
}
