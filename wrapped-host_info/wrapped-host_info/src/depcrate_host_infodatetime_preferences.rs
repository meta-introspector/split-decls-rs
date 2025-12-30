// Generated macro for datetime_preferences (function)
macro_rules! Depcrate_host_infodatetime_preferences {
() => {
// Module: crate::host_info
// Provides: {"datetime_preferences"}
// Dependencies: {}
# [doc = " Retrieves `Preferences` object for `DateTimeFormatter`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " let ue = icu_host_info::datetime_preferences()"] # [doc = "     .expect(\"Failed to retrieve datetime preferences\");"] # [doc = " ```"] # [cfg (feature = "datetime")] pub fn datetime_preferences () -> Result < icu_datetime :: DateTimeFormatterPreferences , HostInfoError > { backends :: Impl :: datetime_preferences () }
};
}
