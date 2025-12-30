// Generated macro for Decoder (struct)
macro_rules! Depcrate_decoderDecoder {
() => {
// Module: crate::decoder
// Provides: {"Decoder"}
// Dependencies: {}
# [doc = " Buffered PEM decoder."] # [doc = ""] # [doc = " Stateful buffered decoder type which decodes an input PEM document according"] # [doc = " to RFC 7468's \"Strict\" grammar."] # [derive (Clone)] pub struct Decoder < 'i > { # [doc = " PEM type label."] type_label : & 'i str , # [doc = " Buffered Base64 decoder."] base64 : Base64Decoder < 'i > , }
};
}
