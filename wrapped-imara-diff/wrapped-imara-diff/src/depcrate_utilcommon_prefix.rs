// Generated macro for common_prefix (function)
macro_rules! Depcrate_utilcommon_prefix {
() => {
// Module: crate::util
// Provides: {"common_prefix"}
// Dependencies: {}
pub fn common_prefix (file1 : & [Token] , file2 : & [Token]) -> u32 { let mut off = 0 ; for (token1 , token2) in file1 . iter () . zip (file2) { if token1 != token2 { break ; } off += 1 ; } off }
};
}
