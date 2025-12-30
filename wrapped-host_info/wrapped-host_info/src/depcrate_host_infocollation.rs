// Generated macro for collation (function)
macro_rules! Depcrate_host_infocollation {
() => {
// Module: crate::host_info
// Provides: {"collation"}
// Dependencies: {}
# [doc = " Retrieves a collation preference."] # [doc = ""] # [doc = " In `::unicode_extensions()` this field is being encoded as `co`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " let collation = icu_host_info::collation()"] # [doc = "     .expect(\"Failed to retrieve collation\");"] # [doc = " ```"] pub fn collation () -> Result < Option < (Language , CollationType) > , HostInfoError > { backends :: Impl :: collation () }
};
}
