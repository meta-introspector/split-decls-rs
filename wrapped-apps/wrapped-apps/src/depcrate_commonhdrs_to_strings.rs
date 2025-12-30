// Generated macro for hdrs_to_strings (function)
macro_rules! Depcrate_commonhdrs_to_strings {
() => {
// Module: crate::common
// Provides: {"hdrs_to_strings"}
// Dependencies: {}
pub fn hdrs_to_strings (hdrs : & [quiche :: h3 :: Header]) -> Vec < (String , String) > { hdrs . iter () . map (| h | { let name = String :: from_utf8_lossy (h . name ()) . to_string () ; let value = String :: from_utf8_lossy (h . value ()) . to_string () ; (name , value) }) . collect () }
};
}
