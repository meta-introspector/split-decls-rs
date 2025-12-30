// Generated macro for impl_238 (impl)
macro_rules! Depcrate_timeimpl_238 {
() => {
// Module: crate::time
// Provides: {"impl_238"}
// Dependencies: {}
impl TryFrom < Duration > for DispatchTime { type Error = () ; fn try_from (value : Duration) -> Result < Self , Self :: Error > { let secs = value . as_secs () as i64 ; secs . checked_mul (1_000_000_000) . and_then (| x | x . checked_add (i64 :: from (value . subsec_nanos ()))) . map (| delta | { Self :: NOW . time (delta) }) . ok_or (()) } }
};
}
