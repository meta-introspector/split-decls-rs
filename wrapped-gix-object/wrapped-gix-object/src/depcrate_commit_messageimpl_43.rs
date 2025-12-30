// Generated macro for impl_43 (impl)
macro_rules! Depcrate_commit_messageimpl_43 {
() => {
// Module: crate::commit::message
// Provides: {"impl_43"}
// Dependencies: {}
impl < 'a > CommitRef < 'a > { # [doc = " Return exactly the same message as [`MessageRef::summary()`]."] pub fn message_summary (& self) -> Cow < 'a , BStr > { summary (self . message) } # [doc = " Return an iterator over message trailers as obtained from the last paragraph of the commit message."] # [doc = " Maybe empty."] pub fn message_trailers (& self) -> body :: Trailers < 'a > { BodyRef :: from_bytes (self . message) . trailers () } }
};
}
