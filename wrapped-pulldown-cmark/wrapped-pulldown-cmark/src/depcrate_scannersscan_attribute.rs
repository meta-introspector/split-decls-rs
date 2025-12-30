// Generated macro for scan_attribute (function)
macro_rules! Depcrate_scannersscan_attribute {
() => {
// Module: crate::scanners
// Provides: {"scan_attribute"}
// Dependencies: {}
# [doc = " Returns the index immediately following the attribute on success."] # [doc = " The argument `buffer_ix` refers to the index into `data` from which we"] # [doc = " should copy into `buffer` when we find bytes to skip."] fn scan_attribute (data : & [u8] , mut ix : usize , newline_handler : Option < & dyn Fn (& [u8]) -> usize > , buffer : & mut Vec < u8 > , buffer_ix : & mut usize ,) -> Option < usize > { ix += scan_attribute_name (& data [ix ..]) ? ; let ix_after_attribute = ix ; ix = scan_whitespace_with_newline_handler_without_buffer (data , ix , newline_handler) ? ; if data . get (ix) == Some (& b'=') { ix = scan_whitespace_with_newline_handler (data , ix_after_attribute , newline_handler , buffer , buffer_ix ,) ? ; ix += 1 ; ix = scan_whitespace_with_newline_handler (data , ix , newline_handler , buffer , buffer_ix) ? ; ix = scan_attribute_value (data , ix , newline_handler , buffer , buffer_ix) ? ; Some (ix) } else { Some (ix_after_attribute) } }
};
}
