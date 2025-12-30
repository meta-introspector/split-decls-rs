// Generated macro for impl_489 (impl)
macro_rules! Depcrate_arg_messageitemimpl_489 {
() => {
// Module: crate::arg::messageitem
// Provides: {"impl_489"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a MessageItem > for & 'a [(MessageItem , MessageItem)] { type Error = () ; fn try_from (i : & 'a MessageItem) -> Result < & 'a [(MessageItem , MessageItem)] , () > { if let MessageItem :: Dict (ref d) = i { Ok (& * d . v) } else { Err (()) } } }
};
}
