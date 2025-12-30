// Generated macro for impl_45 (impl)
macro_rules! Depcrate_commit_messageimpl_45 {
() => {
// Module: crate::commit::message
// Provides: {"impl_45"}
// Dependencies: {}
impl < 'a > MessageRef < 'a > { # [doc = " Parse the given `input` as a message."] # [doc = ""] # [doc = " Note that this cannot fail as everything will be interpreted as title if there is no body separator."] pub fn from_bytes (input : & 'a [u8]) -> Self { let (title , body) = decode :: message (input) ; MessageRef { title , body } } # [doc = " Produce a short commit summary for the message title."] # [doc = ""] # [doc = " This means the following"] # [doc = ""] # [doc = " * Take the subject line which is delimited by two newlines (\\n\\n)"] # [doc = " * transform intermediate consecutive whitespace including \\r into one space"] # [doc = ""] # [doc = " The resulting summary will have folded whitespace before a newline into spaces and stopped that process"] # [doc = " once two consecutive newlines are encountered."] pub fn summary (& self) -> Cow < 'a , BStr > { summary (self . title) } # [doc = " Further parse the body into non-trailer and trailers, which can be iterated from the returned [`BodyRef`]."] pub fn body (& self) -> Option < BodyRef < 'a > > { self . body . map (| b | BodyRef :: from_bytes (b)) } }
};
}
