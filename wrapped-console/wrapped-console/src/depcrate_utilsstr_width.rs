// Generated macro for str_width (function)
macro_rules! Depcrate_utilsstr_width {
() => {
// Module: crate::utils
// Provides: {"str_width"}
// Dependencies: {}
fn str_width (s : & str) -> usize { # [cfg (feature = "unicode-width")] { use unicode_width :: UnicodeWidthStr ; s . width () } # [cfg (not (feature = "unicode-width"))] { s . chars () . count () } }
};
}
