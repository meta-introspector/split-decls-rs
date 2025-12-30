// Generated macro for impl_5 (impl)
macro_rules! Depcrateimpl_5 {
() => {
// Module: crate
// Provides: {"impl_5"}
// Dependencies: {}
impl Default for Options { fn default () -> Self { Options { format : Default :: default () , tree_prefix : None , modification_time : std :: time :: SystemTime :: now () . duration_since (std :: time :: UNIX_EPOCH) . map (| t | t . as_secs () as i64) . unwrap_or_default () , } } }
};
}
