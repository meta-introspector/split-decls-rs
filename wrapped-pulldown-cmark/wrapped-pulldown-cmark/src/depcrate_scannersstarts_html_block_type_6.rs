// Generated macro for starts_html_block_type_6 (function)
macro_rules! Depcrate_scannersstarts_html_block_type_6 {
() => {
// Module: crate::scanners
// Provides: {"starts_html_block_type_6"}
// Dependencies: {}
# [doc = " Assumes `data` is preceded by `<`."] pub (crate) fn starts_html_block_type_6 (data : & [u8]) -> bool { let i = scan_ch (data , b'/') ; let tail = & data [i ..] ; let n = scan_while (tail , is_ascii_alphanumeric) ; if ! is_html_tag (& tail [.. n]) { return false ; } let tail = & tail [n ..] ; tail . is_empty () || tail [0] == b' ' || tail [0] == b'\t' || tail [0] == b'\r' || tail [0] == b'\n' || tail [0] == b'>' || tail . len () >= 2 && & tail [.. 2] == b"/>" }
};
}
