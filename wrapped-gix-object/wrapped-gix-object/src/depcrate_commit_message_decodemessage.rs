// Generated macro for message (function)
macro_rules! Depcrate_commit_message_decodemessage {
() => {
// Module: crate::commit::message::decode
// Provides: {"message"}
// Dependencies: {}
# [doc = " Returns title and body, without separator"] pub fn message (mut input : & [u8]) -> (& BStr , Option < & BStr >) { terminated (subject_and_body :: < () > , eof) . parse_next (& mut input) . expect ("cannot fail") }
};
}
