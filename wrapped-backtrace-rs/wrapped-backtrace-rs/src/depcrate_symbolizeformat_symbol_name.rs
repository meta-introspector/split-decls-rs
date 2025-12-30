// Generated macro for format_symbol_name (function)
macro_rules! Depcrate_symbolizeformat_symbol_name {
() => {
// Module: crate::symbolize
// Provides: {"format_symbol_name"}
// Dependencies: {}
fn format_symbol_name (fmt : fn (& str , & mut fmt :: Formatter < '_ >) -> fmt :: Result , mut bytes : & [u8] , f : & mut fmt :: Formatter < '_ > ,) -> fmt :: Result { while bytes . len () > 0 { match str :: from_utf8 (bytes) { Ok (name) => { fmt (name , f) ? ; break ; } Err (err) => { fmt ("\u{FFFD}" , f) ? ; match err . error_len () { Some (len) => bytes = & bytes [err . valid_up_to () + len ..] , None => break , } } } } Ok (()) }
};
}
