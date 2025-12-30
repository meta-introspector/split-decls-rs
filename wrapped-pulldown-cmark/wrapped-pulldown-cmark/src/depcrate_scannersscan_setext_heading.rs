// Generated macro for scan_setext_heading (function)
macro_rules! Depcrate_scannersscan_setext_heading {
() => {
// Module: crate::scanners
// Provides: {"scan_setext_heading"}
// Dependencies: {}
# [doc = " Scan a setext heading underline."] # [doc = ""] # [doc = " Returns number of bytes in line (including trailing newline) and level."] pub (crate) fn scan_setext_heading (data : & [u8]) -> Option < (usize , HeadingLevel) > { let c = * data . first () ? ; let level = if c == b'=' { HeadingLevel :: H1 } else if c == b'-' { HeadingLevel :: H2 } else { return None ; } ; let mut i = 1 + scan_ch_repeat (& data [1 ..] , c) ; i += scan_blank_line (& data [i ..]) ? ; Some ((i , level)) }
};
}
