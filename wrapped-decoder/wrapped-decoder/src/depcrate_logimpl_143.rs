// Generated macro for impl_143 (impl)
macro_rules! Depcrate_logimpl_143 {
() => {
// Module: crate::log
// Provides: {"impl_143"}
// Dependencies: {}
impl < 'a > DefmtRecord < 'a > { # [doc = " If `record` was produced by [`log_defmt`], returns the corresponding `DefmtRecord`."] pub fn new (log_record : & 'a LogRecord < 'a >) -> Option < Self > { let target = log_record . metadata () . target () ; target . strip_prefix (DEFMT_TARGET_MARKER) . map (| payload | Self { log_record , payload : serde_json :: from_str (payload) . expect ("malformed 'payload'") , }) } # [doc = " Returns the formatted defmt timestamp."] pub fn timestamp (& self) -> & str { self . payload . timestamp . as_str () } pub fn level (& self) -> Option < Level > { self . payload . level } pub fn args (& self) -> & fmt :: Arguments < 'a > { self . log_record . args () } pub fn module_path (& self) -> Option < & 'a str > { self . log_record . module_path () } pub fn file (& self) -> Option < & 'a str > { self . log_record . file () } pub fn line (& self) -> Option < u32 > { self . log_record . line () } }
};
}
