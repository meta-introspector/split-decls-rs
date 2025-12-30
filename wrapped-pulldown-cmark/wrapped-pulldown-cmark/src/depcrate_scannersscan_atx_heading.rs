// Generated macro for scan_atx_heading (function)
macro_rules! Depcrate_scannersscan_atx_heading {
() => {
// Module: crate::scanners
// Provides: {"scan_atx_heading"}
// Dependencies: {}
# [doc = " Scan an ATX heading opening sequence."] # [doc = ""] # [doc = " Returns number of bytes in prefix and level."] pub (crate) fn scan_atx_heading (data : & [u8]) -> Option < HeadingLevel > { let level = scan_ch_repeat (data , b'#') ; if data . get (level) . copied () . map_or (true , is_ascii_whitespace) { HeadingLevel :: try_from (level) . ok () } else { None } }
};
}
