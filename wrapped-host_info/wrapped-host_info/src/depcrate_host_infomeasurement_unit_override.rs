// Generated macro for measurement_unit_override (function)
macro_rules! Depcrate_host_infomeasurement_unit_override {
() => {
// Module: crate::host_info
// Provides: {"measurement_unit_override"}
// Dependencies: {}
# [doc = " Retrieves measurement unit override preference."] # [doc = ""] # [doc = " In `::unicode_extensions()` this field is being encoded as `mu`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " let mu = icu_host_info::measurement_unit_override()"] # [doc = "     .expect(\"Failed to retrieve measurement unit override\");"] # [doc = " ```"] pub fn measurement_unit_override () -> Result < Option < MeasurementUnitOverride > , HostInfoError > { backends :: Impl :: measurement_unit_override () }
};
}
