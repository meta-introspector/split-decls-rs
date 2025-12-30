// Generated macro for params (function)
macro_rules! Depcrate_valueparams {
() => {
// Module: crate::value
// Provides: {"params"}
// Dependencies: {}
pub (crate) fn params (mime : & Mime) -> impl Iterator < Item = (& str , Value) > { mime . params () . map (| (n , v) | { let value = Value :: new (v) . for_name (n) ; (n , value) }) }
};
}
