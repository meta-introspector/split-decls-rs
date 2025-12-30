// Generated macro for impl_264 (impl)
macro_rules! Depcrate_measurementimpl_264 {
() => {
// Module: crate::measurement
// Provides: {"impl_264"}
// Dependencies: {}
impl Measurement for WallTime { type Intermediate = Instant ; type Value = Duration ; fn start (& self) -> Self :: Intermediate { Instant :: now () } fn end (& self , i : Self :: Intermediate) -> Self :: Value { i . elapsed () } fn add (& self , v1 : & Self :: Value , v2 : & Self :: Value) -> Self :: Value { * v1 + * v2 } fn zero (& self) -> Self :: Value { Duration :: from_secs (0) } fn to_f64 (& self , val : & Self :: Value) -> f64 { val . as_nanos () as f64 } fn formatter (& self) -> & dyn ValueFormatter { & DurationFormatter } }
};
}
