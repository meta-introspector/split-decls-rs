// Generated macro for write_pct_encoded_char (function)
macro_rules! Depcrate_percent_encodewrite_pct_encoded_char {
() => {
// Module: crate::percent_encode
// Provides: {"write_pct_encoded_char"}
// Dependencies: {}
# [doc = " Percent-encodes the given character and writes it."] # [inline] fn write_pct_encoded_char < W : fmt :: Write > (writer : & mut W , c : char) -> fmt :: Result { let mut buf = [0_u8 ; 4] ; let buf = c . encode_utf8 (& mut buf) ; buf . bytes () . try_for_each (| b | write ! (writer , "%{:02X}" , b)) }
};
}
