// Generated macro for MeasurementData (struct)
macro_rules! Depcrate_reportMeasurementData {
() => {
// Module: crate::report
// Provides: {"MeasurementData"}
// Dependencies: {}
pub (crate) struct MeasurementData < 'a > { pub data : Data < 'a , f64 , f64 > , pub avg_times : LabeledSample < 'a , f64 > , pub absolute_estimates : Estimates , pub distributions : Distributions , pub comparison : Option < ComparisonData > , pub throughput : Option < Throughput > , }
};
}
