// Generated macro for impl_484 (impl)
macro_rules! Depcrate_arg_messageitemimpl_484 {
() => {
// Module: crate::arg::messageitem
// Provides: {"impl_484"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a MessageItem > for & 'a Signature < 'static > { type Error = () ; fn try_from (i : & 'a MessageItem) -> Result < & 'a Signature < 'static > , () > { if let MessageItem :: Signature (b) = i . peel () { Ok (b) } else { Err (()) } } }
};
}
