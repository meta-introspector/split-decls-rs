// Generated macro for requested_locales (function)
macro_rules! Depcrate_host_inforequested_locales {
() => {
// Module: crate::host_info
// Provides: {"requested_locales"}
// Dependencies: {}
# [doc = " Retrieves an ordered list of locales set as requested by the user in the host"] # [doc = " environment regional preferences."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " let locales = icu_host_info::requested_locales()"] # [doc = "     .expect(\"Failed to retrieve requested locales\");"] # [doc = " ```"] pub fn requested_locales () -> Result < Vec < Locale > , HostInfoError > { backends :: Impl :: requested_locales () }
};
}
