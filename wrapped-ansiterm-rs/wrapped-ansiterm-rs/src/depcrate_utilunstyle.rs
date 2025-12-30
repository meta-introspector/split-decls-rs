// Generated macro for unstyle (function)
macro_rules! Depcrate_utilunstyle {
() => {
// Module: crate::util
// Provides: {"unstyle"}
// Dependencies: {}
# [doc = " Return a concatenated copy of `strs` without the formatting, as an allocated `String`."] pub fn unstyle (strs : & ANSIStrings) -> String { let mut s = String :: new () ; for i in strs . 0 . iter () { s += i . deref () ; } s }
};
}
