// Generated macro for Rewriter (struct)
macro_rules! Depcrate_rewriterRewriter {
() => {
// Module: crate::rewriter
// Provides: {"Rewriter"}
// Dependencies: {}
# [doc = " A rewriter for object and executable files."] # [doc = ""] # [doc = " This struct provides a way to read a file, modify it, and write it back."] # [derive (Debug)] pub struct Rewriter < 'data > { pub (crate) builder : build :: elf :: Builder < 'data > , pub (crate) modified : bool , }
};
}
