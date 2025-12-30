// Generated macro for decode_to_string (function)
macro_rules! Depcrate_punycodedecode_to_string {
() => {
// Module: crate::punycode
// Provides: {"decode_to_string"}
// Dependencies: {}
# [doc = " Convert Punycode to an Unicode `String`."] # [doc = ""] # [doc = " Return None on malformed input or overflow."] # [doc = " Overflow can only happen on inputs that take more than"] # [doc = " 63 encoded bytes, the DNS limit on domain name labels."] # [inline] pub fn decode_to_string (input : & str) -> Option < String > { Some (Decoder :: default () . decode :: < u8 , ExternalCaller > (input . as_bytes ()) . ok () ? . collect () ,) }
};
}
