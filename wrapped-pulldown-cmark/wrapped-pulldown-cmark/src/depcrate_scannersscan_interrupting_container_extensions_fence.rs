// Generated macro for scan_interrupting_container_extensions_fence (function)
macro_rules! Depcrate_scannersscan_interrupting_container_extensions_fence {
() => {
// Module: crate::scanners
// Provides: {"scan_interrupting_container_extensions_fence"}
// Dependencies: {}
pub (crate) fn scan_interrupting_container_extensions_fence (data : & [u8]) -> bool { let fence_length = scan_ch_repeat (data , b':') ; let kind_start = fence_length + scan_whitespace_no_nl (& data [fence_length ..]) ; let kind_length = scan_while (& data [kind_start ..] , | c | { is_ascii_alphanumeric (c) || c == b'_' || c == b'-' || c == b':' || c == b'.' }) ; if fence_length > 2 && kind_length > 0 { true } else { false } }
};
}
