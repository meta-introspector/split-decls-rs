// Generated macro for impl_79 (impl)
macro_rules! Depcrate_messagesimpl_79 {
() => {
// Module: crate::messages
// Provides: {"impl_79"}
// Dependencies: {}
impl < R : BufRead > Iterator for MessageIter < R > { type Item = io :: Result < Message > ; fn next (& mut self) -> Option < Self :: Item > { let mut line = String :: new () ; self . input . read_line (& mut line) . map (| n | { if n == 0 { None } else { if line . ends_with ('\n') { line . truncate (line . len () - 1) ; } let mut deserializer = serde_json :: Deserializer :: from_str (& line) ; deserializer . disable_recursion_limit () ; Some (Message :: deserialize (& mut deserializer) . unwrap_or (Message :: TextLine (line))) } }) . transpose () } }
};
}
