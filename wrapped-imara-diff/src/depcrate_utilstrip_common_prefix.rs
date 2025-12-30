// Generated macro for strip_common_prefix (function)
macro_rules! Depcrate_utilstrip_common_prefix {
() => {
// Module: crate::util
// Provides: {"strip_common_prefix"}
// Dependencies: {}
pub fn strip_common_prefix (file1 : & mut & [Token] , file2 : & mut & [Token]) -> u32 { let off = common_prefix (file1 , file2) ; * file1 = & file1 [off as usize ..] ; * file2 = & file2 [off as usize ..] ; off }
};
}
