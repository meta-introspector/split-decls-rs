// Generated macro for region (function)
macro_rules! Depcrate_host_inforegion {
() => {
// Module: crate::host_info
// Provides: {"region"}
// Dependencies: {}
# [doc = " Retrieves a region set in the host environment regional preferences."] # [doc = ""] # [doc = " That region may be already populated into `requested_locales` or not, depending"] # [doc = " on the host."] # [doc = " In `::unicode_extensions()` this field is being encoded as `rg`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " let region = icu_host_info::region()"] # [doc = "     .expect(\"Failed to retrieve region\");"] # [doc = " ```"] pub fn region () -> Result < Option < Region > , HostInfoError > { backends :: Impl :: region () }
};
}
