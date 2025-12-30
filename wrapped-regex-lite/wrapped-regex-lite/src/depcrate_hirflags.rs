// Generated macro for Flags (struct)
macro_rules! Depcrate_hirFlags {
() => {
// Module: crate::hir
// Provides: {"Flags"}
// Dependencies: {}
# [doc = " Various flags that control the interpretation of the pattern."] # [doc = ""] # [doc = " These can be set via explicit configuration in code, or change dynamically"] # [doc = " during parsing via inline flags. For example, `foo(?i:bar)baz` will match"] # [doc = " `foo` and `baz` case sensitively and `bar` case insensitively (assuming a"] # [doc = " default configuration)."] # [derive (Clone , Copy , Debug , Default)] pub (crate) struct Flags { # [doc = " Whether to match case insensitively."] # [doc = ""] # [doc = " This is the `i` flag."] pub (crate) case_insensitive : bool , # [doc = " Whether `^` and `$` should be treated as line anchors or not."] # [doc = ""] # [doc = " This is the `m` flag."] pub (crate) multi_line : bool , # [doc = " Whether `.` should match line terminators or not."] # [doc = ""] # [doc = " This is the `s` flag."] pub (crate) dot_matches_new_line : bool , # [doc = " Whether to swap the meaning of greedy and non-greedy operators."] # [doc = ""] # [doc = " This is the `U` flag."] pub (crate) swap_greed : bool , # [doc = " Whether to enable CRLF mode."] # [doc = ""] # [doc = " This is the `R` flag."] pub (crate) crlf : bool , # [doc = " Whether to ignore whitespace. i.e., verbose mode."] # [doc = ""] # [doc = " This is the `x` flag."] pub (crate) ignore_whitespace : bool , }
};
}
