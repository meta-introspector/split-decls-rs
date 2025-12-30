// Generated macro for is_defmt_frame (function)
macro_rules! Depcrate_logis_defmt_frame {
() => {
// Module: crate::log
// Provides: {"is_defmt_frame"}
// Dependencies: {}
# [doc = " Determines whether `metadata` belongs to a log record produced by [`log_defmt`]."] pub fn is_defmt_frame (metadata : & Metadata) -> bool { metadata . target () . starts_with (DEFMT_TARGET_MARKER) }
};
}
