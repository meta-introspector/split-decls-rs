// Generated macro for is_utf8 (function)
macro_rules! Depcrateis_utf8 {
() => {
// Module: crate
// Provides: {"is_utf8"}
// Dependencies: {}
# [doc = " Check if given bytes contain valid UTF8 string"] pub fn is_utf8 (data : & [u8]) -> bool { std :: str :: from_utf8 (data) . is_ok () }
};
}
