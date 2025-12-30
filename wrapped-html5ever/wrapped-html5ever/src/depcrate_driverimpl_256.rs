// Generated macro for impl_256 (impl)
macro_rules! Depcrate_driverimpl_256 {
() => {
// Module: crate::driver
// Provides: {"impl_256"}
// Dependencies: {}
impl < Sink : TreeSink > Parser < Sink > { # [doc = " Wrap this parser into a `TendrilSink` that accepts UTF-8 bytes."] # [doc = ""] # [doc = " Use this when your input is bytes that are known to be in the UTF-8 encoding."] # [doc = " Decoding is lossy, like `String::from_utf8_lossy`."] pub fn from_utf8 (self) -> Utf8LossyDecoder < Self > { Utf8LossyDecoder :: new (self) } # [doc = " Wrap this parser into a `TendrilSink` that accepts bytes"] # [doc = " and tries to detect the correct character encoding."] # [doc = ""] # [doc = " Currently this looks for a Byte Order Mark,"] # [doc = " then uses `BytesOpts::transport_layer_encoding`,"] # [doc = " then falls back to UTF-8."] # [doc = ""] # [doc = " FIXME(https://github.com/servo/html5ever/issues/18): this should look for `<meta>` elements"] # [doc = " and other data per"] # [doc = " https://html.spec.whatwg.org/multipage/syntax.html#determining-the-character-encoding"] pub fn from_bytes (self , opts : BytesOpts) -> BytesParser < Sink > { BytesParser { state : BytesParserState :: Initial { parser : self } , opts : opts , } } }
};
}
