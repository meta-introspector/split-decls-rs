// Generated macro for FormatterConfig (struct)
macro_rules! Depcrate_log_formatFormatterConfig {
() => {
// Module: crate::log::format
// Provides: {"FormatterConfig"}
// Dependencies: {}
# [doc = " Settings that control how defmt frames should be formatted."] # [derive (Debug , Default)] pub struct FormatterConfig < 'a > { # [doc = " The format to use"] pub format : FormatterFormat < 'a > , # [doc = " If `true`, then the logs should include a timestamp."] # [doc = ""] # [doc = " Not all targets can supply a timestamp, and if not, it should be"] # [doc = " omitted."] pub is_timestamp_available : bool , }
};
}
