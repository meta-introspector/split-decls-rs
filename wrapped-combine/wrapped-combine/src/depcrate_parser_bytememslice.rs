// Generated macro for memslice (function)
macro_rules! Depcrate_parser_bytememslice {
() => {
// Module: crate::parser::byte
// Provides: {"memslice"}
// Dependencies: {}
fn memslice (needle : & [u8] , haystack : & [u8]) -> Option < usize > { let (& prefix , suffix) = match needle . split_first () { Some (x) => x , None => return Some (0) , } ; for i in memchr :: memchr_iter (prefix , haystack) { if haystack [i + 1 ..] . starts_with (suffix) { return Some (i) ; } } None }
};
}
