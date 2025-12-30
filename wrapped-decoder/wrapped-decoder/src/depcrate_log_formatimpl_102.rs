// Generated macro for impl_102 (impl)
macro_rules! Depcrate_log_formatimpl_102 {
() => {
// Module: crate::log::format
// Provides: {"impl_102"}
// Dependencies: {}
impl < 'a > FormatterConfig < 'a > { # [doc = " Create a new custom formatter config."] # [doc = ""] # [doc = " This allows the user to supply a custom log-format string. See the"] # [doc = " \"Custom Log Output\" section of the defmt book for details of the format."] pub fn custom (format : & 'a str) -> Self { FormatterConfig { format : FormatterFormat :: from_string (format , true) . unwrap_or (FormatterFormat :: Custom (format)) , is_timestamp_available : false , } } # [doc = " Modify a formatter configuration, setting the 'timestamp available' flag"] # [doc = " to true."] pub fn with_timestamp (mut self) -> Self { self . is_timestamp_available = true ; self } # [doc = " Modify a formatter configuration, setting the 'with_location' flag"] # [doc = " to true."] # [doc = ""] # [doc = " Do not use this with a custom log formatter."] pub fn with_location (mut self) -> Self { match self . format { FormatterFormat :: OneLine { with_location : _ } => { self . format = FormatterFormat :: OneLine { with_location : true , } ; self } FormatterFormat :: Default { with_location : _ } => { self . format = FormatterFormat :: Default { with_location : true , } ; self } _ => self , } } }
};
}
