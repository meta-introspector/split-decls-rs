// Generated macro for impl_905 (impl)
macro_rules! Depcrate_timestampimpl_905 {
() => {
// Module: crate::timestamp
// Provides: {"impl_905"}
// Dependencies: {}
impl From < SignedDuration > for TimestampArithmetic { fn from (sdur : SignedDuration) -> TimestampArithmetic { let duration = Duration :: from (sdur) ; TimestampArithmetic { duration } } }
};
}
