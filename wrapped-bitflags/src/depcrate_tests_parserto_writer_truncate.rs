// Generated macro for to_writer_truncate (function)
macro_rules! Depcrate_tests_parserto_writer_truncate {
() => {
// Module: crate::tests::parser
// Provides: {"to_writer_truncate"}
// Dependencies: {}
# [doc = "\nWrite a flags value as text, ignoring any unknown bits.\n"] pub fn to_writer_truncate < B : Flags > (flags : & B , writer : impl Write) -> Result < () , fmt :: Error > where B :: Bits : WriteHex , { to_writer (& B :: from_bits_truncate (flags . bits ()) , writer) }
};
}
