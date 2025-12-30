// Generated macro for trim_xml_start (function)
macro_rules! Depcrate_utilstrim_xml_start {
() => {
// Module: crate::utils
// Provides: {"trim_xml_start"}
// Dependencies: {}
# [doc = " Returns a byte slice with leading XML whitespace bytes removed."] # [doc = ""] # [doc = " 'Whitespace' refers to the definition used by [`is_whitespace`]."] # [inline] pub const fn trim_xml_start (mut bytes : & [u8]) -> & [u8] { while let [first , rest @ ..] = bytes { if is_whitespace (* first) { bytes = rest ; } else { break ; } } bytes }
};
}
