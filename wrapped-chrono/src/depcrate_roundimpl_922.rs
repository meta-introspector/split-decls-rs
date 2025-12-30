// Generated macro for impl_922 (impl)
macro_rules! Depcrate_roundimpl_922 {
() => {
// Module: crate::round
// Provides: {"impl_922"}
// Dependencies: {}
impl fmt :: Display for RoundingError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match * self { RoundingError :: DurationExceedsTimestamp => { write ! (f , "duration in nanoseconds exceeds timestamp") } RoundingError :: DurationExceedsLimit => { write ! (f , "duration exceeds num_nanoseconds limit") } RoundingError :: TimestampExceedsLimit => { write ! (f , "timestamp exceeds num_nanoseconds limit") } } } }
};
}
