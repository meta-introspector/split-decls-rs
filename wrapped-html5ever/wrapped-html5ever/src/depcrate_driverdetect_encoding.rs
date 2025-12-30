// Generated macro for detect_encoding (function)
macro_rules! Depcrate_driverdetect_encoding {
() => {
// Module: crate::driver
// Provides: {"detect_encoding"}
// Dependencies: {}
# [doc = " https://html.spec.whatwg.org/multipage/syntax.html#determining-the-character-encoding"] fn detect_encoding (bytes : & ByteTendril , opts : & BytesOpts) -> EncodingRef { if bytes . starts_with (b"\xEF\xBB\xBF") { return encoding :: all :: UTF_8 } if bytes . starts_with (b"\xFE\xFF") { return encoding :: all :: UTF_16BE } if bytes . starts_with (b"\xFF\xFE") { return encoding :: all :: UTF_16LE } if let Some (encoding) = opts . transport_layer_encoding { return encoding } return encoding :: all :: UTF_8 }
};
}
