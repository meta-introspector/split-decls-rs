// Generated macro for impl_552 (impl)
macro_rules! Depcrate_common_retry_afterimpl_552 {
() => {
// Module: crate::common::retry_after
// Provides: {"impl_552"}
// Dependencies: {}
impl TryFromValues for After { fn try_from_values < 'i , I > (values : & mut I) -> Result < Self , Error > where I : Iterator < Item = & 'i HeaderValue > , { values . next () . and_then (| val | { if let Some (delay) = Seconds :: from_val (val) { return Some (After :: Delay (delay)) ; } let date = HttpDate :: from_val (val) ? ; Some (After :: DateTime (date)) }) . ok_or_else (Error :: invalid) } }
};
}
