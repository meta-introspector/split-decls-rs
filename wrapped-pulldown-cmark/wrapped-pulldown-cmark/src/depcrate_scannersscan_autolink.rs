// Generated macro for scan_autolink (function)
macro_rules! Depcrate_scannersscan_autolink {
() => {
// Module: crate::scanners
// Provides: {"scan_autolink"}
// Dependencies: {}
# [doc = " Returns (next_byte_offset, uri, type)"] pub (crate) fn scan_autolink (text : & str , start_ix : usize) -> Option < (usize , CowStr < '_ > , LinkType) > { scan_uri (text , start_ix) . map (| (bytes , uri) | (bytes , uri , LinkType :: Autolink)) . or_else (| | scan_email (text , start_ix) . map (| (bytes , uri) | (bytes , uri , LinkType :: Email))) }
};
}
