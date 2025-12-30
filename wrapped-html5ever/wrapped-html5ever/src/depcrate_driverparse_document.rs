// Generated macro for parse_document (function)
macro_rules! Depcrate_driverparse_document {
() => {
// Module: crate::driver
// Provides: {"parse_document"}
// Dependencies: {}
# [doc = " Parse an HTML document"] # [doc = ""] # [doc = " The returned value implements `tendril::TendrilSink`"] # [doc = " so that Unicode input may be provided incrementally,"] # [doc = " or all at once with the `one` method."] # [doc = ""] # [doc = " If your input is bytes, use `Parser::from_utf8` or `Parser::from_bytes`."] pub fn parse_document < Sink > (sink : Sink , opts : ParseOpts) -> Parser < Sink > where Sink : TreeSink { let tb = TreeBuilder :: new (sink , opts . tree_builder) ; let tok = Tokenizer :: new (tb , opts . tokenizer) ; Parser { tokenizer : tok } }
};
}
