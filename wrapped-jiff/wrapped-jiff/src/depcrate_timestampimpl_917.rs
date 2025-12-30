// Generated macro for impl_917 (impl)
macro_rules! Depcrate_timestampimpl_917 {
() => {
// Module: crate::timestamp
// Provides: {"impl_917"}
// Dependencies: {}
impl < 'a > From < (Unit , & 'a Zoned) > for TimestampDifference { # [inline] fn from ((largest , zdt) : (Unit , & 'a Zoned)) -> TimestampDifference { TimestampDifference :: from ((largest , Timestamp :: from (zdt))) } }
};
}
