// Generated macro for find_char (function)
macro_rules! Depcrate_uts46find_char {
() => {
// Module: crate::uts46
// Provides: {"find_char"}
// Dependencies: {}
fn find_char (codepoint : char) -> & 'static Mapping { let r = TABLE . binary_search_by (| ref range | { if codepoint > range . to { Less } else if codepoint < range . from { Greater } else { Equal } }) ; r . ok () . map (| i | & TABLE [i] . mapping) . unwrap () }
};
}
