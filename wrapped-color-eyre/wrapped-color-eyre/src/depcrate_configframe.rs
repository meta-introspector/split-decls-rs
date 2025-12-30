// Generated macro for Frame (struct)
macro_rules! Depcrate_configFrame {
() => {
// Module: crate::config
// Provides: {"Frame"}
// Dependencies: {}
# [doc = " A representation of a Frame from a Backtrace or a SpanTrace"] # [derive (Debug)] # [non_exhaustive] pub struct Frame { # [doc = " Frame index"] pub n : usize , # [doc = " frame symbol name"] pub name : Option < String > , # [doc = " source line number"] pub lineno : Option < u32 > , # [doc = " source file path"] pub filename : Option < PathBuf > , }
};
}
