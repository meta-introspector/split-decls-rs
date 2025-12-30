// Generated macro for CargoParser (trait)
macro_rules! Depcrate_commandCargoParser {
() => {
// Module: crate::command
// Provides: {"CargoParser"}
// Dependencies: {}
# [doc = " Cargo output is structured as one JSON per line. This trait abstracts parsing one line of"] # [doc = " cargo output into a Rust data type"] pub (crate) trait CargoParser < T > : Send + 'static { fn from_line (& self , line : & str , error : & mut String) -> Option < T > ; fn from_eof (& self) -> Option < T > ; }
};
}
