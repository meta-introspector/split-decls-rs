// Generated macro for parse_unicode_version (function)
macro_rules! Depcrate_parseparse_unicode_version {
() => {
// Module: crate::parse
// Provides: {"parse_unicode_version"}
// Dependencies: {}
fn parse_unicode_version (filename : & str , contents : & str) -> (u8 , u8 , u8) { let (name , extension) = filename . rsplit_once ('.') . expect ("Failed to split file name into name and extension") ; let re = Regex :: new (& format ! (r"^# {name}-(\d+)\.(\d+)\.(\d+)\.{extension}\n")) . unwrap () ; let caps = re . captures (contents) . expect ("Failed to find unicode version in unicode data") ; let (_ , [major , minor , patch]) = caps . extract () ; [major , minor , patch] . map (| s | s . parse () . expect ("Failed to parse unicode version")) . into () }
};
}
