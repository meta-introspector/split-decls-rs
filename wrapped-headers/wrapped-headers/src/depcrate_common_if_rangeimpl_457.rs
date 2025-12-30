// Generated macro for impl_457 (impl)
macro_rules! Depcrate_common_if_rangeimpl_457 {
() => {
// Module: crate::common::if_range
// Provides: {"impl_457"}
// Dependencies: {}
impl < 'a > From < & 'a IfRange_ > for HeaderValue { fn from (if_range : & 'a IfRange_) -> HeaderValue { match * if_range { IfRange_ :: EntityTag (ref tag) => tag . into () , IfRange_ :: Date (ref date) => date . into () , } } }
};
}
