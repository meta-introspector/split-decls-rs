// Generated macro for common_postfix (function)
macro_rules! Depcrate_utilcommon_postfix {
() => {
// Module: crate::util
// Provides: {"common_postfix"}
// Dependencies: {}
pub fn common_postfix (file1 : & [Token] , file2 : & [Token]) -> u32 { let mut off = 0 ; for (token1 , token2) in file1 . iter () . rev () . zip (file2 . iter () . rev ()) { if token1 != token2 { break ; } off += 1 ; } off }
};
}
