// Generated macro for hour_cycle (function)
macro_rules! Depcrate_host_infohour_cycle {
() => {
// Module: crate::host_info
// Provides: {"hour_cycle"}
// Dependencies: {}
# [doc = " Retrieves an hour_cycle preference."] # [doc = ""] # [doc = " In `::unicode_extensions()` this field is being encoded as `hc`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " let region = icu_host_info::hour_cycle()"] # [doc = "     .expect(\"Failed to retrieve hour cycle\");"] # [doc = " ```"] pub fn hour_cycle () -> Result < Option < HourCycle > , HostInfoError > { backends :: Impl :: hour_cycle () }
};
}
