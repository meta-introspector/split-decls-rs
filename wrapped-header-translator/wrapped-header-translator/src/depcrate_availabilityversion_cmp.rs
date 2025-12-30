// Generated macro for version_cmp (function)
macro_rules! Depcrate_availabilityversion_cmp {
() => {
// Module: crate::availability
// Provides: {"version_cmp"}
// Dependencies: {}
fn version_cmp (left : Version , right : Version) -> Ordering { left . x . cmp (& right . x) . then_with (| | left . y . unwrap_or (0) . cmp (& right . y . unwrap_or (0))) . then_with (| | left . z . unwrap_or (0) . cmp (& right . z . unwrap_or (0))) }
};
}
