// Generated macro for expect_utf8 (function)
macro_rules! Depcrate_hooksexpect_utf8 {
() => {
// Module: crate::hooks
// Provides: {"expect_utf8"}
// Dependencies: {}
unsafe fn expect_utf8 < 'a > (p_str : * const c_char , description : & 'static str) -> & 'a str { expect_optional_utf8 (p_str , description) . unwrap_or_else (| | panic ! ("received empty {description}")) }
};
}
