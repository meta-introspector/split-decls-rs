// Generated macro for param (function)
macro_rules! Depcrate_valueparam {
() => {
// Module: crate::value
// Provides: {"param"}
// Dependencies: {}
pub (crate) fn param < 'a > (mime : & 'a Mime , key : & str) -> Option < Value < 'a > > { params (mime) . find (| e | key == e . 0) . map (| e | e . 1) }
};
}
