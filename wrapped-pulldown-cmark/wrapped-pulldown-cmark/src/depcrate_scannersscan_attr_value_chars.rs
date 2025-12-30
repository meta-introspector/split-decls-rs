// Generated macro for scan_attr_value_chars (function)
macro_rules! Depcrate_scannersscan_attr_value_chars {
() => {
// Module: crate::scanners
// Provides: {"scan_attr_value_chars"}
// Dependencies: {}
fn scan_attr_value_chars (data : & [u8]) -> usize { scan_while (data , is_valid_unquoted_attr_value_char) }
};
}
