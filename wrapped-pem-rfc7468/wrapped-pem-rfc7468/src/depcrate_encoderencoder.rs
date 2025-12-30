// Generated macro for Encoder (struct)
macro_rules! Depcrate_encoderEncoder {
() => {
// Module: crate::encoder
// Provides: {"Encoder"}
// Dependencies: {}
# [doc = " Buffered PEM encoder."] # [doc = ""] # [doc = " Stateful buffered encoder type which encodes an input PEM document according"] # [doc = " to RFC 7468's \"Strict\" grammar."] pub struct Encoder < 'l , 'o > { # [doc = " PEM type label."] type_label : & 'l str , # [doc = " Line ending used to wrap Base64."] line_ending : LineEnding , # [doc = " Buffered Base64 encoder."] base64 : Base64Encoder < 'o > , }
};
}
