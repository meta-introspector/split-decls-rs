// Generated macro for total_message_size (function)
macro_rules! Depcrate_messagetotal_message_size {
() => {
// Module: crate::message
// Provides: {"total_message_size"}
// Dependencies: {}
pub fn total_message_size (buf : & [u8]) -> Result < usize , DemarshalError > { message_start_parse (buf) . map (| x | x . total_size) }
};
}
