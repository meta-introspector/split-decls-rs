// Generated macro for impl_481 (impl)
macro_rules! Depcrate_arg_messageitemimpl_481 {
() => {
// Module: crate::arg::messageitem
// Provides: {"impl_481"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a MessageItem > for & 'a str { type Error = () ; fn try_from (i : & 'a MessageItem) -> Result < & 'a str , Self :: Error > { match i . peel () { MessageItem :: Str (ref b) => Ok (b) , MessageItem :: ObjectPath (ref b) => Ok (b) , MessageItem :: Signature (ref b) => Ok (b) , _ => Err (()) , } } }
};
}
