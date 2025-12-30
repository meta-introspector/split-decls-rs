// Generated macro for is_printable_ascii (function)
macro_rules! Depcrateis_printable_ascii {
() => {
// Module: crate
// Provides: {"is_printable_ascii"}
// Dependencies: {}
# [doc = " Check if given bytes contain only printable ASCII characters"] pub fn is_printable_ascii (data : & [u8]) -> bool { for & char in data { if ! (0x20 ..= 0x7e) . contains (& char) { return false ; } } true }
};
}
