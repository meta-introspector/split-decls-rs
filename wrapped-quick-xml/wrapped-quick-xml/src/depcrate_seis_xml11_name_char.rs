// Generated macro for is_xml11_name_char (function)
macro_rules! Depcrate_seis_xml11_name_char {
() => {
// Module: crate::se
// Provides: {"is_xml11_name_char"}
// Dependencies: {}
# [doc = " <https://www.w3.org/TR/xml11/#NT-NameChar>"] const fn is_xml11_name_char (ch : char) -> bool { match ch { '-' | '.' | '0' ..= '9' | '\u{00B7}' | '\u{0300}' ..= '\u{036F}' | '\u{203F}' ..= '\u{2040}' => { true } _ => is_xml11_name_start_char (ch) , } }
};
}
