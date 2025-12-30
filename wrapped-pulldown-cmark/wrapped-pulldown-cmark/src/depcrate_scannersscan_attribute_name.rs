// Generated macro for scan_attribute_name (function)
macro_rules! Depcrate_scannersscan_attribute_name {
() => {
// Module: crate::scanners
// Provides: {"scan_attribute_name"}
// Dependencies: {}
# [doc = " Returns bytes scanned"] fn scan_attribute_name (data : & [u8]) -> Option < usize > { let (& c , tail) = data . split_first () ? ; if is_ascii_alpha (c) || c == b'_' || c == b':' { Some (1 + scan_while (tail , | c | { is_ascii_alphanumeric (c) || c == b'_' || c == b'.' || c == b':' || c == b'-' }) ,) } else { None } }
};
}
