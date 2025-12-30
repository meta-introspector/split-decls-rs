// Generated macro for impl_482 (impl)
macro_rules! Depcrate_arg_messageitemimpl_482 {
() => {
// Module: crate::arg::messageitem
// Provides: {"impl_482"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a MessageItem > for & 'a String { type Error = () ; fn try_from (i : & 'a MessageItem) -> Result < & 'a String , () > { if let MessageItem :: Str (b) = i . peel () { Ok (b) } else { Err (()) } } }
};
}
