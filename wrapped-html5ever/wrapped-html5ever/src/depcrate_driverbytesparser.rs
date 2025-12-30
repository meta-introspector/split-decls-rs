// Generated macro for BytesParser (struct)
macro_rules! Depcrate_driverBytesParser {
() => {
// Module: crate::driver
// Provides: {"BytesParser"}
// Dependencies: {}
# [doc = " An HTML parser,"] # [doc = " ready to recieve bytes input through the `tendril::TendrilSink` trait’s methods."] # [doc = ""] # [doc = " See `Parser::from_bytes`."] pub struct BytesParser < Sink > where Sink : TreeSink { state : BytesParserState < Sink > , opts : BytesOpts , }
};
}
