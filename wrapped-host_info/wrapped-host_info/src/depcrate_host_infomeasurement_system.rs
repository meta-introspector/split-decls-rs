// Generated macro for measurement_system (function)
macro_rules! Depcrate_host_infomeasurement_system {
() => {
// Module: crate::host_info
// Provides: {"measurement_system"}
// Dependencies: {}
# [doc = " Retrieves a measurement system preference."] # [doc = ""] # [doc = " In `::unicode_extensions()` this field is being encoded as `ms`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " let ms = icu_host_info::measurement_system()"] # [doc = "     .expect(\"Failed to retrieve measurement system\");"] # [doc = " ```"] pub fn measurement_system () -> Result < Option < MeasurementSystem > , HostInfoError > { backends :: Impl :: measurement_system () }
};
}
