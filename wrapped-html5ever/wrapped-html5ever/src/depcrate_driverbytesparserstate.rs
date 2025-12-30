// Generated macro for BytesParserState (enum)
macro_rules! Depcrate_driverBytesParserState {
() => {
// Module: crate::driver
// Provides: {"BytesParserState"}
// Dependencies: {}
enum BytesParserState < Sink > where Sink : TreeSink { Initial { parser : Parser < Sink > , } , Buffering { parser : Parser < Sink > , buffer : ByteTendril } , Parsing { decoder : LossyDecoder < Parser < Sink > > , } , Transient }
};
}
