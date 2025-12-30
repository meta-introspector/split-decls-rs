// Generated macro for impl_201 (impl)
macro_rules! Depcrate_civil_timeimpl_201 {
() => {
// Module: crate::civil::time
// Provides: {"impl_201"}
// Dependencies: {}
impl From < SignedDuration > for TimeArithmetic { fn from (sdur : SignedDuration) -> TimeArithmetic { let duration = Duration :: from (sdur) ; TimeArithmetic { duration } } }
};
}
