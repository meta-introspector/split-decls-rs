// Generated macro for calendar (function)
macro_rules! Depcrate_host_infocalendar {
() => {
// Module: crate::host_info
// Provides: {"calendar"}
// Dependencies: {}
# [doc = " Retrieves a calendar preference."] # [doc = ""] # [doc = " In `::unicode_extensions()` this field is being encoded as `ca`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " let calendar = icu_host_info::calendar()"] # [doc = "     .expect(\"Failed to retrieve calendar\");"] # [doc = " ```"] pub fn calendar () -> Result < Option < CalendarAlgorithm > , HostInfoError > { backends :: Impl :: calendar () }
};
}
