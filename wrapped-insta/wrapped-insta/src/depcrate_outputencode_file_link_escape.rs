// Generated macro for encode_file_link_escape (function)
macro_rules! Depcrate_outputencode_file_link_escape {
() => {
// Module: crate::output
// Provides: {"encode_file_link_escape"}
// Dependencies: {}
# [doc = " Encodes a path as an OSC-8 escape sequence. This makes it a clickable link in supported"] # [doc = " terminal emulators."] fn encode_file_link_escape (path : & Path) -> String { assert ! (path . is_absolute ()) ; format ! ("\x1b]8;;file://{}\x1b\\{}\x1b]8;;\x1b\\" , path . display () , path . display ()) }
};
}
