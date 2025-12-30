// Generated macro for debug_checked_utf8_unchecked (function)
macro_rules! Depcrate_stringdebug_checked_utf8_unchecked {
() => {
// Module: crate::string
// Provides: {"debug_checked_utf8_unchecked"}
// Dependencies: {}
# [track_caller] unsafe fn debug_checked_utf8_unchecked (bytes : & [u8]) -> & str { if cfg ! (debug_assertions) { match str :: from_utf8 (bytes) { Ok (s) => s , Err (err) => panic ! ("unsafe precondition violated: CF function did not return valid UTF-8: {err}") , } } else { unsafe { str :: from_utf8_unchecked (bytes) } } }
};
}
