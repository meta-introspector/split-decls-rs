// Generated macro for impl_1010 (impl)
macro_rules! Depcrate_tz_offsetimpl_1010 {
() => {
// Module: crate::tz::offset
// Provides: {"impl_1010"}
// Dependencies: {}
impl From < SignedDuration > for OffsetArithmetic { fn from (sdur : SignedDuration) -> OffsetArithmetic { let duration = Duration :: from (sdur) ; OffsetArithmetic { duration } } }
};
}
