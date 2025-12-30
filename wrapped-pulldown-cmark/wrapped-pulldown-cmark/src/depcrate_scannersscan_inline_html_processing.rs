// Generated macro for scan_inline_html_processing (function)
macro_rules! Depcrate_scannersscan_inline_html_processing {
() => {
// Module: crate::scanners
// Provides: {"scan_inline_html_processing"}
// Dependencies: {}
# [doc = " Scan processing directive, with initial \"<?\" already consumed."] # [doc = " Returns the next byte offset on success."] pub (crate) fn scan_inline_html_processing (bytes : & [u8] , mut ix : usize , scan_guard : & mut HtmlScanGuard ,) -> Option < usize > { if ix <= scan_guard . processing { return None ; } while let Some (offset) = memchr (b'?' , & bytes [ix ..]) { ix += offset + 1 ; if bytes . get (ix) == Some (& b'>') { return Some (ix + 1) ; } } scan_guard . processing = ix ; None }
};
}
