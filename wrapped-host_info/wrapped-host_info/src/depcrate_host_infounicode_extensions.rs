// Generated macro for unicode_extensions (function)
macro_rules! Depcrate_host_infounicode_extensions {
() => {
// Module: crate::host_info
// Provides: {"unicode_extensions"}
// Dependencies: {}
# [doc = " Retrieves `Unicode` extensions struct populated from host regional preferences."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " let ue = icu_host_info::unicode_extensions()"] # [doc = "     .expect(\"Failed to retrieve host info\");"] # [doc = " ```"] pub fn unicode_extensions () -> Result < Unicode , HostInfoError > { backends :: Impl :: unicode_extensions () }
};
}
