// Generated macro for convert_osstr_to_utf16 (function)
macro_rules! Depcrate_processconvert_osstr_to_utf16 {
() => {
// Module: crate::process
// Provides: {"convert_osstr_to_utf16"}
// Dependencies: {}
fn convert_osstr_to_utf16 (s : & OsStr) -> Vec < u16 > { let mut bytes : Vec < _ > = s . encode_wide () . collect () ; bytes . push (0) ; bytes }
};
}
