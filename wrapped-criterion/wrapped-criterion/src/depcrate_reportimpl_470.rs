// Generated macro for impl_470 (impl)
macro_rules! Depcrate_reportimpl_470 {
() => {
// Module: crate::report
// Provides: {"impl_470"}
// Dependencies: {}
impl < 'a > MeasurementData < 'a > { pub fn iter_counts (& self) -> & Sample < f64 > { self . data . x () } # [cfg (feature = "csv_output")] pub fn sample_times (& self) -> & Sample < f64 > { self . data . y () } }
};
}
