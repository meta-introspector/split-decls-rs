// Generated macro for find_header (function)
macro_rules! Depcratefind_header {
() => {
// Module: crate
// Provides: {"find_header"}
// Dependencies: {}
fn find_header < 'a > (header_list : & 'a mut [Header] , header : & Header ,) -> Option < & 'a mut Header > { header_list . iter_mut () . find (| curr | curr . name () == header . name ()) }
};
}
