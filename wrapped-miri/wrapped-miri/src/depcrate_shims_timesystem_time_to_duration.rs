// Generated macro for system_time_to_duration (function)
macro_rules! Depcrate_shims_timesystem_time_to_duration {
() => {
// Module: crate::shims::time
// Provides: {"system_time_to_duration"}
// Dependencies: {}
# [doc = " Returns the time elapsed between the provided time and the unix epoch as a `Duration`."] pub fn system_time_to_duration < 'tcx > (time : & SystemTime) -> InterpResult < 'tcx , Duration > { time . duration_since (SystemTime :: UNIX_EPOCH) . map_err (| _ | err_unsup_format ! ("times before the Unix epoch are not supported")) . into () }
};
}
