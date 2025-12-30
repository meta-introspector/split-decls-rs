// Generated macro for timestamp (function)
macro_rules! Depcrate_exporttimestamp {
() => {
// Module: crate::export
// Provides: {"timestamp"}
// Dependencies: {}
# [cfg (not (feature = "unstable-test"))] # [inline (always)] pub fn timestamp (fmt : crate :: Formatter < '_ >) { extern "Rust" { fn _defmt_timestamp (_ : crate :: Formatter < '_ >) ; } unsafe { _defmt_timestamp (fmt) } }
};
}
