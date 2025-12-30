// Generated macro for rfind (function)
macro_rules! Depcrate_bytesetrfind {
() => {
// Module: crate::byteset
// Provides: {"rfind"}
// Dependencies: {}
# [inline] pub (crate) fn rfind (haystack : & [u8] , byteset : & [u8]) -> Option < usize > { match byteset . len () { 0 => None , 1 => memrchr (byteset [0] , haystack) , 2 => memrchr2 (byteset [0] , byteset [1] , haystack) , 3 => memrchr3 (byteset [0] , byteset [1] , byteset [2] , haystack) , _ => { let table = build_table (byteset) ; scalar :: reverse_search_bytes (haystack , | b | table [b as usize] != 0) } } }
};
}
