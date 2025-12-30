// Generated macro for impl_456 (impl)
macro_rules! Depcrate_common_if_rangeimpl_456 {
() => {
// Module: crate::common::if_range
// Provides: {"impl_456"}
// Dependencies: {}
impl TryFromValues for IfRange_ { fn try_from_values < 'i , I > (values : & mut I) -> Result < Self , Error > where I : Iterator < Item = & 'i HeaderValue > , { values . next () . and_then (| val | { if let Some (tag) = EntityTag :: from_val (val) { return Some (IfRange_ :: EntityTag (tag)) ; } let date = HttpDate :: from_val (val) ? ; Some (IfRange_ :: Date (date)) }) . ok_or_else (Error :: invalid) } }
};
}
