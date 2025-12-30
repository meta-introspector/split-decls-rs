// Generated macro for impl_487 (impl)
macro_rules! Depcrate_arg_messageitemimpl_487 {
() => {
// Module: crate::arg::messageitem
// Provides: {"impl_487"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a MessageItem > for & 'a [MessageItem] { type Error = () ; fn try_from (i : & 'a MessageItem) -> Result < & 'a [MessageItem] , () > { i . inner :: < & Vec < MessageItem > > () . map (| s | & * * s) } }
};
}
