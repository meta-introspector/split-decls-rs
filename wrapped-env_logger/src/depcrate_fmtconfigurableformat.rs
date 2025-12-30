// Generated macro for ConfigurableFormat (struct)
macro_rules! Depcrate_fmtConfigurableFormat {
() => {
// Module: crate::fmt
// Provides: {"ConfigurableFormat"}
// Dependencies: {}
# [doc = " A [custom format][crate::Builder::format] with settings for which fields to show"] pub struct ConfigurableFormat { pub (crate) timestamp : Option < TimestampPrecision > , pub (crate) module_path : bool , pub (crate) target : bool , pub (crate) level : bool , pub (crate) source_file : bool , pub (crate) source_line_number : bool , pub (crate) indent : Option < usize > , pub (crate) suffix : & 'static str , # [cfg (feature = "kv")] pub (crate) kv_format : Option < Box < KvFormatFn > > , }
};
}
