// Generated macro for impl_619 (impl)
macro_rules! Depcrate_sourceimpl_619 {
() => {
// Module: crate::source
// Provides: {"impl_619"}
// Dependencies: {}
impl SourceFileRange { # [doc = " Attempts to get the text from the source file. This can fail if the source text isn't"] # [doc = " loaded."] pub fn as_str (& self) -> Option < & str > { (self . sf . src . as_ref () . map (| src | src . as_str ())) . or_else (| | self . sf . external_src . get () ? . get_source ()) . and_then (| x | x . get (self . range . clone ())) } }
};
}
