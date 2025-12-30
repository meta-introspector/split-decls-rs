// Generated macro for impl_914 (impl)
macro_rules! Depcrate_timestampimpl_914 {
() => {
// Module: crate::timestamp
// Provides: {"impl_914"}
// Dependencies: {}
impl < 'a > From < & 'a Zoned > for TimestampDifference { # [inline] fn from (zdt : & 'a Zoned) -> TimestampDifference { TimestampDifference :: from (Timestamp :: from (zdt)) } }
};
}
