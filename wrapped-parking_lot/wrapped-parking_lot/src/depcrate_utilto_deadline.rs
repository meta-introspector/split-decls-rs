// Generated macro for to_deadline (function)
macro_rules! Depcrate_utilto_deadline {
() => {
// Module: crate::util
// Provides: {"to_deadline"}
// Dependencies: {}
# [inline] pub fn to_deadline (timeout : Duration) -> Option < Instant > { Instant :: now () . checked_add (timeout) }
};
}
