// Generated macro for Config (struct)
macro_rules! Depcrate_hirConfig {
() => {
// Module: crate::hir
// Provides: {"Config"}
// Dependencies: {}
# [doc = " The configuration for a regex parser."] # [derive (Clone , Copy , Debug)] pub (crate) struct Config { # [doc = " The maximum number of times we're allowed to recurse."] # [doc = ""] # [doc = " Note that unlike the regex-syntax parser, we actually use recursion in"] # [doc = " this parser for simplicity. My hope is that by setting a conservative"] # [doc = " default call limit and providing a way to configure it, that we can"] # [doc = " keep this simplification. But if we must, we can re-work the parser to"] # [doc = " put the call stack on the heap like regex-syntax does."] pub (crate) nest_limit : u32 , # [doc = " Various flags that control how a pattern is interpreted."] pub (crate) flags : Flags , }
};
}
