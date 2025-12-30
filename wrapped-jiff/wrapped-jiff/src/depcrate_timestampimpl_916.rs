// Generated macro for impl_916 (impl)
macro_rules! Depcrate_timestampimpl_916 {
() => {
// Module: crate::timestamp
// Provides: {"impl_916"}
// Dependencies: {}
impl From < (Unit , Zoned) > for TimestampDifference { # [inline] fn from ((largest , zdt) : (Unit , Zoned)) -> TimestampDifference { TimestampDifference :: from ((largest , Timestamp :: from (zdt))) } }
};
}
