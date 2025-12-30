// Generated macro for get_offset (function)
macro_rules! Depcrate_errorget_offset {
() => {
// Module: crate::error
// Provides: {"get_offset"}
// Dependencies: {}
# [doc = " Find the line number and column of the target string within the source string. Will panic if"] # [doc = " target is not a substring of source."] pub (crate) fn get_offset (source : & str , target : & str) -> (usize , usize) { let offset = target . as_ptr () as isize - source . as_ptr () as isize ; let to_scan = & source [0 .. (offset as usize)] ; let mut line = 1 ; let mut column = 0 ; for byte in to_scan . bytes () { match byte as char { '\n' => { line += 1 ; column = 0 ; } _ => { column += 1 ; } } } (line , column) }
};
}
