// Generated macro for trim_xml_end (function)
macro_rules! Depcrate_utilstrim_xml_end {
() => {
// Module: crate::utils
// Provides: {"trim_xml_end"}
// Dependencies: {}
# [doc = " Returns a byte slice with trailing XML whitespace bytes removed."] # [doc = ""] # [doc = " 'Whitespace' refers to the definition used by [`is_whitespace`]."] # [inline] pub const fn trim_xml_end (mut bytes : & [u8]) -> & [u8] { while let [rest @ .. , last] = bytes { if is_whitespace (* last) { bytes = rest ; } else { break ; } } bytes }
};
}
