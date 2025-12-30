// Generated macro for Parser (struct)
macro_rules! Depcrate_driverParser {
() => {
// Module: crate::driver
// Provides: {"Parser"}
// Dependencies: {}
# [doc = " An HTML parser,"] # [doc = " ready to receive Unicode input through the `tendril::TendrilSink` trait’s methods."] pub struct Parser < Sink > where Sink : TreeSink , { pub tokenizer : Tokenizer < TreeBuilder < Sink :: Handle , Sink > > , pub input_buffer : BufferQueue , }
};
}
