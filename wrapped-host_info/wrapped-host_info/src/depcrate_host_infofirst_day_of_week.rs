// Generated macro for first_day_of_week (function)
macro_rules! Depcrate_host_infofirst_day_of_week {
() => {
// Module: crate::host_info
// Provides: {"first_day_of_week"}
// Dependencies: {}
# [doc = " Retrieves a first day of week preference."] # [doc = ""] # [doc = " In `::unicode_extensions()` this field is being encoded as `fd`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " let fd = icu_host_info::first_day_of_week()"] # [doc = "     .expect(\"Failed to retrieve first day of week\");"] # [doc = " ```"] pub fn first_day_of_week () -> Result < Option < FirstDay > , HostInfoError > { backends :: Impl :: first_day_of_week () }
};
}
