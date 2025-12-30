// Generated macro for scan_inline_html_comment (function)
macro_rules! Depcrate_scannersscan_inline_html_comment {
() => {
// Module: crate::scanners
// Provides: {"scan_inline_html_comment"}
// Dependencies: {}
# [doc = " Scan comment, declaration, or CDATA section, with initial \"<!\" already consumed."] # [doc = " Returns byte offset on match."] pub (crate) fn scan_inline_html_comment (bytes : & [u8] , mut ix : usize , scan_guard : & mut HtmlScanGuard ,) -> Option < usize > { let c = * bytes . get (ix) ? ; ix += 1 ; match c { b'-' if ix > scan_guard . comment => { if * bytes . get (ix) ? != b'-' { return None ; } ix -= 1 ; while let Some (x) = memchr (b'-' , & bytes [ix ..]) { ix += x + 1 ; scan_guard . comment = ix ; if bytes . get (ix) == Some (& b'-') && bytes . get (ix + 1) == Some (& b'>') { return Some (ix + 2) ; } } None } b'[' if bytes [ix ..] . starts_with (b"CDATA[") && ix > scan_guard . cdata => { ix += b"CDATA[" . len () ; ix = memchr (b']' , & bytes [ix ..]) . map_or (bytes . len () , | x | ix + x) ; let close_brackets = scan_ch_repeat (& bytes [ix ..] , b']') ; ix += close_brackets ; if close_brackets == 0 || bytes . get (ix) != Some (& b'>') { scan_guard . cdata = ix ; None } else { Some (ix + 1) } } _ if c . is_ascii_alphabetic () && ix > scan_guard . declaration => { ix = memchr (b'>' , & bytes [ix ..]) . map_or (bytes . len () , | x | ix + x) ; if bytes . get (ix) != Some (& b'>') { scan_guard . declaration = ix ; None } else { Some (ix + 1) } } _ => None , } }
};
}
