// Generated macro for SsrRule (struct)
macro_rules! DepcrateSsrRule {
() => {
// Module: crate
// Provides: {"SsrRule"}
// Dependencies: {}
# [derive (Debug)] pub struct SsrRule { # [doc = " A structured pattern that we're searching for."] pattern : parsing :: RawPattern , # [doc = " What we'll replace it with."] template : parsing :: RawPattern , parsed_rules : Vec < parsing :: ParsedRule > , }
};
}
