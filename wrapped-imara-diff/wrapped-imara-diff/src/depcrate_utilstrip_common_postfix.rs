// Generated macro for strip_common_postfix (function)
macro_rules! Depcrate_utilstrip_common_postfix {
() => {
// Module: crate::util
// Provides: {"strip_common_postfix"}
// Dependencies: {}
pub fn strip_common_postfix (file1 : & mut & [Token] , file2 : & mut & [Token]) -> u32 { let off = common_postfix (file1 , file2) ; * file1 = & file1 [.. file1 . len () - off as usize] ; * file2 = & file2 [.. file2 . len () - off as usize] ; off }
};
}
