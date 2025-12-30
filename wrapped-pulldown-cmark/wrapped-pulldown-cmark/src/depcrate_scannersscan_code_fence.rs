// Generated macro for scan_code_fence (function)
macro_rules! Depcrate_scannersscan_code_fence {
() => {
// Module: crate::scanners
// Provides: {"scan_code_fence"}
// Dependencies: {}
# [doc = " Scan code fence."] # [doc = ""] # [doc = " Returns number of bytes scanned and the char that is repeated to make the code fence."] pub (crate) fn scan_code_fence (data : & [u8]) -> Option < (usize , u8) > { let c = * data . first () ? ; if ! (c == b'`' || c == b'~') { return None ; } let i = 1 + scan_ch_repeat (& data [1 ..] , c) ; if i >= 3 { if c == b'`' { let suffix = & data [i ..] ; let next_line = i + scan_nextline (suffix) ; if suffix [.. (next_line - i)] . iter () . any (| & b | b == b'`') { return None ; } } Some ((i , c)) } else { None } }
};
}
