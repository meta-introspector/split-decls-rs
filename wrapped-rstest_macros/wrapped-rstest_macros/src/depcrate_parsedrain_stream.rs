// Generated macro for drain_stream (function)
macro_rules! Depcrate_parsedrain_stream {
() => {
// Module: crate::parse
// Provides: {"drain_stream"}
// Dependencies: {}
# [allow (dead_code)] pub (crate) fn drain_stream (input : ParseStream) { let _ = input . step (| cursor | { let mut rest = * cursor ; while let Some ((_ , next)) = rest . token_tree () { rest = next } Ok ((() , rest)) }) ; }
};
}
