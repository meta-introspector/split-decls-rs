// Generated macro for parse_fragment (function)
macro_rules! Depcrate_driverparse_fragment {
() => {
// Module: crate::driver
// Provides: {"parse_fragment"}
// Dependencies: {}
# [doc = " Parse an HTML fragment"] # [doc = ""] # [doc = " The returned value implements `tendril::TendrilSink`"] # [doc = " so that Unicode input may be provided incrementally,"] # [doc = " or all at once with the `one` method."] # [doc = ""] # [doc = " If your input is bytes, use `Parser::from_utf8`."] pub fn parse_fragment < Sink > (sink : Sink , opts : ParseOpts , context_name : QualName , context_attrs : Vec < Attribute > ,) -> Parser < Sink > where Sink : TreeSink , { let context_elem = create_element (& sink , context_name , context_attrs) ; parse_fragment_for_element (sink , opts , context_elem , None) }
};
}
