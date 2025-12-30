// Generated macro for ConfigurableFormatWriter (struct)
macro_rules! Depcrate_fmtConfigurableFormatWriter {
() => {
// Module: crate::fmt
// Provides: {"ConfigurableFormatWriter"}
// Dependencies: {}
# [doc = " The default format."] # [doc = ""] # [doc = " This format needs to work with any combination of crate features."] struct ConfigurableFormatWriter < 'a > { format : & 'a ConfigurableFormat , buf : & 'a mut Formatter , written_header_value : bool , }
};
}
