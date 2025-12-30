// Generated macro for impl_906 (impl)
macro_rules! Depcrate_timestampimpl_906 {
() => {
// Module: crate::timestamp
// Provides: {"impl_906"}
// Dependencies: {}
impl From < UnsignedDuration > for TimestampArithmetic { fn from (udur : UnsignedDuration) -> TimestampArithmetic { let duration = Duration :: from (udur) ; TimestampArithmetic { duration } } }
};
}
